// this test is vibe coded bc im too lazy to write any test but i was scared i make some silly
// mistakes yknow :p

#[cfg(test)]
pub mod tests {
    use crate::models::field::Field;
    use crate::models::field::FieldType;
    use crate::models::field::InsertField;
    use crate::models::field::MigrationKind;
    use crate::models::field::MigrationStrategy;
    use crate::models::field::RelationConfig;
    use crate::models::field::SelectConfig;
    use crate::models::ids::TableId;
    use surrealdb::sql::Thing;

    fn sample_table_id() -> TableId {
        TableId(Thing::from(("table", "1")))
    }

    fn sample_select_field() -> FieldType {
        FieldType::Select {
            config: SelectConfig {
                options: vec!["a".into(), "b".into()],
                multiple: true,
            },
        }
    }

    fn sample_relation_field(table: TableId) -> FieldType {
        FieldType::Relation {
            config: RelationConfig {
                table,
                multiple: false,
            },
        }
    }

    #[test]
    fn test_scalar_detection() {
        assert!(FieldType::Text.is_scalar());
        assert!(FieldType::Number.is_scalar());
        assert!(FieldType::Bool.is_scalar());
        assert!(FieldType::Date.is_scalar());

        assert!(!sample_select_field().is_scalar());
        assert!(!sample_relation_field(sample_table_id()).is_scalar());
    }

    #[test]
    fn test_allows_multiple() {
        let select = FieldType::Select {
            config: SelectConfig {
                options: vec!["x".into()],
                multiple: true,
            },
        };
        let relation = FieldType::Relation {
            config: RelationConfig {
                table: sample_table_id(),
                multiple: false,
            },
        };
        assert!(select.allows_multiple());
        assert!(!relation.allows_multiple());
    }

    #[test]
    fn test_field_validation() {
        let valid_select = sample_select_field();
        assert!(valid_select.validate().is_ok());

        let invalid_select = FieldType::Select {
            config: SelectConfig {
                options: vec![],
                multiple: false,
            },
        };
        assert!(invalid_select.validate().is_err());

        let relation = sample_relation_field(sample_table_id());
        assert!(relation.validate().is_ok());
    }

    #[test]
    fn test_migration_kind() {
        let text = FieldType::Text;
        let number = FieldType::Number;
        let select = sample_select_field();
        let relation1 = sample_relation_field(TableId(Thing::from(("table", "1"))));
        let relation2 = sample_relation_field(TableId(Thing::from(("table", "2"))));

        assert_eq!(text.migration_kind_to(&text), MigrationKind::Safe);
        assert_eq!(text.migration_kind_to(&number), MigrationKind::NeedsMap);
        assert_eq!(text.migration_kind_to(&select), MigrationKind::NeedsMap);

        assert_eq!(relation1.migration_kind_to(&relation1), MigrationKind::Safe);
        assert_eq!(
            relation1.migration_kind_to(&relation2),
            MigrationKind::Risky
        );

        assert_eq!(
            text.migration_kind_to(&relation1),
            MigrationKind::Impossible
        );
    }

    #[test]
    fn test_field_from_insert() {
        let insert = InsertField {
            table: sample_table_id(),
            name: "Test Field".into(),
            field_type: FieldType::Text,
            order: 1,
        };

        let field = Field::from_insert(insert);
        assert_eq!(field.name, "Test Field");
        assert_eq!(field.order, 1);
        assert!(field.id.is_none());
        assert!(field.created_at.timestamp() > 0);
        assert!(field.updated_at.timestamp() > 0);
    }

    #[test]
    fn test_migration_kind_helpers() {
        assert!(MigrationKind::Safe.is_allowed());
        assert!(MigrationKind::NeedsMap.is_allowed());
        assert!(MigrationKind::Risky.is_allowed());
        assert!(!MigrationKind::Impossible.is_allowed());

        assert!(MigrationKind::NeedsMap.requires_user_input());
        assert!(MigrationKind::Risky.requires_user_input());
        assert!(!MigrationKind::Safe.requires_user_input());
        assert!(!MigrationKind::Impossible.requires_user_input());
    }
    #[test]
    fn test_text_to_number_migration() {
        // Simulate a Text field with string numbers
        let field_type_from = FieldType::Text;
        let field_type_to = FieldType::Number;

        let values: Vec<String> = vec!["42".into(), "100".into(), "0".into()];
        let expected: Vec<f64> = vec![42.0, 100.0, 0.0];

        // Migration strategy: Cast (try to convert automatically)
        let strategy = MigrationStrategy::Cast;

        // Apply migration
        let migrated: Vec<f64> = values
            .iter()
            .map(|v| match strategy {
                MigrationStrategy::Cast => v.parse::<f64>().unwrap(),
                _ => panic!("Other strategies not implemented in this test"),
            })
            .collect();

        // Check the result
        assert_eq!(migrated, expected);
    }

    #[test]
    #[should_panic]
    fn test_text_to_number_invalid_migration() {
        // Text values that cannot be converted to numbers
        let values: Vec<String> = vec!["hello".into(), "world".into()];
        let strategy = MigrationStrategy::Cast;

        let _migrated: Vec<f64> = values
            .iter()
            .map(|v| match strategy {
                MigrationStrategy::Cast => v.parse::<f64>().unwrap(), // will panic
                _ => panic!("Other strategies not implemented"),
            })
            .collect();
    }

    #[test]
    fn test_text_to_select_migration() {
        // Simulate Text -> Select migration with mapping
        let field_type_from = FieldType::Text;
        let field_type_to = FieldType::Select {
            config: SelectConfig {
                options: vec!["A".into(), "B".into(), "C".into()],
                multiple: false,
            },
        };

        let values: Vec<String> = vec!["A".into(), "B".into(), "A".into()];
        let mapping = |v: &str| -> usize {
            match v {
                "A" => 0,
                "B" => 1,
                "C" => 2,
                _ => panic!("Invalid mapping"),
            }
        };

        let migrated: Vec<usize> = values.iter().map(|v| mapping(v)).collect();
        let expected = vec![0, 1, 0];
        assert_eq!(migrated, expected);
    }
    #[test]
    fn test_text_to_number_cast() {
        // Simulated Text field values
        let values: Vec<&str> = vec!["42", "100", "0"];
        let expected: Vec<f64> = vec![42.0, 100.0, 0.0];

        // Conversion: Text -> Number using "Cast"
        let migrated: Vec<f64> = values.iter().map(|v| v.parse::<f64>().unwrap()).collect();

        assert_eq!(migrated, expected);
    }

    #[test]
    #[should_panic]
    fn test_text_to_number_cast_invalid() {
        // Text field with an invalid number
        let values: Vec<&str> = vec!["42", "hello", "100"];
        let _migrated: Vec<f64> = values
            .iter()
            .map(|v| v.parse::<f64>().unwrap()) // should panic
            .collect();
    }

    #[test]
    fn test_text_to_select_mapping() {
        let values: Vec<&str> = vec!["A", "B", "A"];
        let options = vec!["A", "B", "C"];

        // Mapping: Text -> Select index
        let mapping = |v: &str| -> usize { options.iter().position(|o| *o == v).unwrap() };

        let migrated: Vec<usize> = values.iter().map(|v| mapping(v)).collect();
        let expected = vec![0, 1, 0];

        assert_eq!(migrated, expected);
    }

    #[test]
    fn test_number_to_text_cast() {
        let values: Vec<f64> = vec![1.0, 2.0, 3.0];
        let expected: Vec<String> = vec!["1".into(), "2".into(), "3".into()];

        let migrated: Vec<String> = values.iter().map(|v| v.to_string()).collect();

        assert_eq!(migrated, expected);
    }

    #[test]
    fn test_bool_to_text_cast() {
        let values: Vec<bool> = vec![true, false, true];
        let expected: Vec<String> = vec!["true".into(), "false".into(), "true".into()];

        let migrated: Vec<String> = values.iter().map(|v| v.to_string()).collect();

        assert_eq!(migrated, expected);
    }
}
