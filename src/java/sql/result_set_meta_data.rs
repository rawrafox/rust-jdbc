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
  
  // int 	getColumnDisplaySize(int column)
  // String 	getColumnLabel(int column)
  // String 	getColumnName(int column)
  // int 	getColumnType(int column)
  // String 	getColumnTypeName(int column)
  // int 	getPrecision(int column)
  // int 	getScale(int column)
  // String 	getSchemaName(int column)
  // String 	getTableName(int column)
  // boolean 	isAutoIncrement(int column)
  // boolean 	isCaseSensitive(int column)
  // boolean 	isCurrency(int column)
  // boolean 	isDefinitelyWritable(int column)
  // int 	isNullable(int column)
  // boolean 	isReadOnly(int column)
  // boolean 	isSearchable(int column)
  // boolean 	isSigned(int column)
  // boolean 	isWritable(int column)
}

impl TResultSetMetaData for ResultSetMetaData {}
