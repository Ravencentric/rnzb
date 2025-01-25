use crate::segment::Segment;
use crate::tuple::Tuple;
use chrono::{DateTime, Utc};
use nzb_rs::{File as RustFile, Segment as RustSegment};
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::fmt::Display;

// Python wrapper class for File
#[pyclass(module = "rnzb", frozen, eq, hash)]
#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub(crate) struct File {
    #[pyo3(get)]
    poster: String,
    #[pyo3(get)]
    datetime: DateTime<Utc>,
    #[pyo3(get)]
    subject: String,
    #[pyo3(get)]
    groups: Tuple<String>,
    #[pyo3(get)]
    segments: Tuple<Segment>,
    #[serde(skip)]
    inner: RustFile,
}

// Implement Python-esque debug
impl Debug for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "File(poster={:?}, datetime={:?}, subject={:?}, groups={:?}, segments={:?})",
            self.poster,
            self.datetime.to_rfc3339(),
            self.subject,
            self.groups,
            self.segments
        )
    }
}

// Implement Python-esque display
// It's identical to Debug
impl Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self, f)
    }
}

// Implement conversion from RustFile to File
impl From<RustFile> for File {
    fn from(f: RustFile) -> Self {
        Self {
            poster: f.poster.clone(),
            datetime: f.datetime,
            subject: f.subject.clone(),
            groups: f.groups.clone().into(),
            segments: f
                .segments
                .clone()
                .into_iter()
                .map(Into::into)
                .collect::<Vec<_>>()
                .into(),
            inner: f,
        }
    }
}

// Implement conversion from File to RustFile
impl From<File> for RustFile {
    fn from(f: File) -> Self {
        RustFile::new(
            f.poster.clone(),
            f.datetime,
            f.subject.clone(),
            f.groups.0.clone(),
            f.segments.0.into_iter().map(Into::into).collect::<Vec<RustSegment>>(),
        )
    }
}

#[pymethods]
impl File {
    #[new]
    #[pyo3(signature = (*, poster, datetime, subject, groups, segments))]
    fn __new__(
        poster: String,
        datetime: DateTime<Utc>,
        subject: String,
        groups: Vec<String>,
        segments: Vec<Segment>,
    ) -> Self {
        Self {
            poster: poster.clone(),
            datetime,
            subject: subject.clone(),
            groups: groups.clone().into(),
            segments: segments.clone().into(),
            inner: RustFile::new(
                poster,
                datetime,
                subject,
                groups,
                segments.into_iter().map(Into::into).collect::<Vec<RustSegment>>(),
            ),
        }
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self)
    }

    fn __str__(&self) -> String {
        self.__repr__()
    }

    // Size of the file calculated from the sum of segment sizes.
    #[getter]
    fn size(&self) -> u64 {
        self.inner.size()
    }

    // Complete name of the file with it's extension extracted from the subject.
    // May return [`None`] if it fails to extract the name.
    #[getter]
    fn name(&self) -> Option<&str> {
        self.inner.name()
    }

    // Base name of the file without it's extension extracted from the [`File::name`].
    // May return [`None`] if it fails to extract the stem.
    #[getter]
    fn stem(&self) -> Option<&str> {
        self.inner.stem()
    }

    //  Extension of the file extracted from the [`File::name`].
    // May return [`None`] if it fails to extract the extension.
    #[getter]
    fn extension(&self) -> Option<&str> {
        self.inner.extension()
    }

    // Return [`true`] if the file is a `.par2` file, [`false`] otherwise.
    fn is_par2(&self) -> bool {
        self.inner.is_par2()
    }

    // Return [`true`] if the file is a `.rar` file, [`false`] otherwise.
    fn is_rar(&self) -> bool {
        self.inner.is_rar()
    }

    // Return [`true`] if the file is obfuscated, [`false`] otherwise.
    fn is_obfuscated(&self) -> bool {
        self.inner.is_obfuscated()
    }
}
