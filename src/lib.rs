mod model;
mod parser;


pub use model::PlWordNet;


#[cfg(test)]
mod tests {
    use crate::model::PlWordNet;

    #[test]
    fn loading() {
        let wn = PlWordNet::from_file("plwordnet_4_2.xml").unwrap();
        assert_eq!(wn.lexical_units.len(), 513410);
        assert_eq!(wn.synsets.len(), 353585);
        assert_eq!(wn.relation_types.len(), 306);
        assert_eq!(wn.synset_relations.len(), 1477851);
        assert_eq!(wn.lexical_relations.len(), 393137);
    }
}
