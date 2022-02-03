#[cfg(test)]
mod tests {
    use bson::spec::ElementType;
    use crate::mongo_util;

    #[test]
    fn doc_creation() {
        let test_doc = mongo_util::create_doc(10, 0, 10, false, 0, 1000);
        assert_eq!(test_doc.get("_id").unwrap().as_str(), Some("w-0-seq-1000"));
        // assert_eq!(test_doc.get("fld0").unwrap().element_type(), ElementType::Int32); //TODO Debug why this does not work sometimes.
        assert_eq!(test_doc.get("fld1").unwrap().element_type(), ElementType::DateTime);
        assert_eq!(test_doc.get("fld2").unwrap().as_str(), Some("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do."));
    }
}