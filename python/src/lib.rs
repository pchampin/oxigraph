#![allow(clippy::zero_ptr, clippy::transmute_ptr_to_ptr)]

use cpython::exc::ValueError;
use cpython::*;
use oxigraph::model;
use oxigraph::Error;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;

py_module_initializer!(oxigraph, initoxigraph, PyInit_oxigraph, |py, m| {
    m.add(py, "__doc__", "Oxigraph Python bindings")?;
    m.add_class::<NamedNode>(py)?;
    m.add_class::<BlankNode>(py)?;
    m.add_class::<Literal>(py)?;
    Ok(())
});

fn new_value_error(py: Python<'_>, error: &Error) -> PyErr {
    PyErr::new::<ValueError, _>(py, error.to_string())
}

fn eq_compare<T: Eq + Ord>(a: &T, b: &T, op: &CompareOp) -> bool {
    match op {
        CompareOp::Lt => a < b,
        CompareOp::Le => a <= b,
        CompareOp::Eq => a == b,
        CompareOp::Ne => a != b,
        CompareOp::Gt => a > b,
        CompareOp::Ge => a >= b,
    }
}

fn hash(t: &impl Hash) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

py_class!(class NamedNode |py| {
    data inner: model::NamedNode;

    def __new__(_cls, value: &str) -> PyResult<NamedNode> {
        Self::create_instance(py, model::NamedNode::parse(value).map_err(|error| new_value_error(py, &error))?)
    }

    def value(&self) -> PyResult<PyString> {
        Ok(PyString::new(py, self.inner(py).as_str()))
    }

    def __str__(&self) -> PyResult<String> {
        Ok(self.inner(py).to_string())
    }

    def __richcmp__(&self, other: &NamedNode, op: CompareOp) -> PyResult<bool> {
        Ok(eq_compare(&self.inner(py), &other.inner(py), &op))
    }

    def __hash__(&self) -> PyResult<u64> {
        Ok(hash(self.inner(py)))
    }
});

py_class!(class BlankNode |py| {
    data inner: model::BlankNode;

    def __new__(_cls) -> PyResult<BlankNode> {
        Self::create_instance(py, model::BlankNode::default())
    }

    def __str__(&self) -> PyResult<String> {
        Ok(self.inner(py).to_string())
    }

    def __richcmp__(&self, other: &BlankNode, op: CompareOp) -> PyResult<bool> {
        Ok(eq_compare(&self.inner(py), &other.inner(py), &op))
    }

    def __hash__(&self) -> PyResult<u64> {
        Ok(hash(self.inner(py)))
    }
});

py_class!(class Literal |py| {
    data inner: model::Literal;

    def __new__(_cls, value: String, language: Option<String> = None, datatype: Option<NamedNode> = None) -> PyResult<Literal> {
        Self::create_instance(py, if let Some(language) = language {
            model::Literal::new_language_tagged_literal(value, language)
        } else if let Some(datatype) = datatype {
            model::Literal::new_typed_literal(value, datatype.inner(py).clone())
        } else {
            model::Literal::new_simple_literal(value)
        })
    }

    def value(&self) -> PyResult<PyString> {
        Ok(PyString::new(py, self.inner(py).value()))
    }

    def language(&self) -> PyResult<Option<PyString>> {
        Ok(self.inner(py).language().map(|l| PyString::new(py, l)))
    }

    def datatype(&self) -> PyResult<NamedNode> {
        NamedNode::create_instance(py, self.inner(py).datatype().clone())
    }

    def __str__(&self) -> PyResult<String> {
        Ok(self.inner(py).to_string())
    }

    def __richcmp__(&self, other: &Literal, op: CompareOp) -> PyResult<bool> {
        Ok(eq_compare(&self.inner(py), &other.inner(py), &op))
    }

    def __hash__(&self) -> PyResult<u64> {
        Ok(hash(self.inner(py)))
    }
});
