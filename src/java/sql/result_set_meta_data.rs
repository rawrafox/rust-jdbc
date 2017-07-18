use std;
use java;
use java::lang::IObject;
use jvm;
use jvm::ToValue;

jvm_object!(ResultSetMetaData, "java/sql/ResultSetMetaData");

pub trait TResultSetMetaData : IObject /* + TWrapper */ {
  fn get_catalog_name(&self, column: i32) -> java::Result<Option<std::string::String>> {
    return jvm_call!(string: self, "getCatalogName", "(I)Ljava/lang/String;", &[&column.to_value()]);
  }

  fn get_column_class_name(&self, column: i32) -> java::Result<Option<std::string::String>> {
    return jvm_call!(string: self, "getColumnClassName", "(I)Ljava/lang/String;", &[&column.to_value()]);
  }

  fn get_column_count(&self) -> java::Result<i32> {
    return jvm_call!(int: self, "getColumnCount", "()I", &[]);
  }

  fn get_column_display_size(&self, column: i32) -> java::Result<i32> {
    return jvm_call!(int: self, "getColumnDisplaySize", "(I)I", &[&column.to_value()]);
  }

  fn get_column_label(&self, column: i32) -> java::Result<Option<std::string::String>> {
    return jvm_call!(string: self, "getColumnLabel", "(I)Ljava/lang/String;", &[&column.to_value()]);
  }

  fn get_column_name(&self, column: i32) -> java::Result<Option<std::string::String>> {
    return jvm_call!(string: self, "getColumnName", "(I)Ljava/lang/String;", &[&column.to_value()]);
  }

  fn get_column_type(&self, column: i32) -> java::Result<i32> {
    return jvm_call!(int: self, "getColumnType", "(I)I", &[&column.to_value()]);
  }

  fn get_column_type_name(&self, column: i32) -> java::Result<Option<std::string::String>> {
    return jvm_call!(string: self, "getColumnTypeName", "(I)Ljava/lang/String;", &[&column.to_value()]);
  }

  fn get_precision(&self, column: i32) -> java::Result<i32> {
    return jvm_call!(int: self, "getPrecision", "(I)I", &[&column.to_value()]);
  }

  fn get_scale(&self, column: i32) -> java::Result<i32> {
    return jvm_call!(int: self, "getScale", "(I)I", &[&column.to_value()]);
  }

  fn get_schema_name(&self, column: i32) -> java::Result<Option<std::string::String>> {
    return jvm_call!(string: self, "getSchemaName", "(I)Ljava/lang/String;", &[&column.to_value()]);
  }

  fn get_table_name(&self, column: i32) -> java::Result<Option<std::string::String>> {
    return jvm_call!(string: self, "getTableName", "(I)Ljava/lang/String;", &[&column.to_value()]);
  }

  fn is_auto_increment(&self, column: i32) -> java::Result<bool> {
    return jvm_call!(bool: self, "isAutoIncrement", "(I)Z", &[&column.to_value()]);
  }

  fn is_case_sensitive(&self, column: i32) -> java::Result<bool> {
    return jvm_call!(bool: self, "isCaseSensitive", "(I)Z", &[&column.to_value()]);
  }

  fn is_currency(&self, column: i32) -> java::Result<bool> {
    return jvm_call!(bool: self, "isCurrency", "(I)Z", &[&column.to_value()]);
  }

  fn is_definitely_writable(&self, column: i32) -> java::Result<bool> {
    return jvm_call!(bool: self, "isDefinitelyWritable", "(I)Z", &[&column.to_value()]);
  }

  fn is_nullable(&self, column: i32) -> java::Result<i32> {
    return jvm_call!(int: self, "isNullable", "(I)I", &[&column.to_value()]);
  }

  fn is_read_only(&self, column: i32) -> java::Result<bool> {
    return jvm_call!(bool: self, "isReadOnly", "(I)Z", &[&column.to_value()]);
  }

  fn is_searchable(&self, column: i32) -> java::Result<bool> {
    return jvm_call!(bool: self, "isSearchable", "(I)Z", &[&column.to_value()]);
  }

  fn is_signed(&self, column: i32) -> java::Result<bool> {
    return jvm_call!(bool: self, "isSigned", "(I)Z", &[&column.to_value()]);
  }

  fn is_writable(&self, column: i32) -> java::Result<bool> {
    return jvm_call!(bool: self, "isWritable", "(I)Z", &[&column.to_value()]);
  }
}

impl TResultSetMetaData for ResultSetMetaData {}
