use serde::{Deserialize, Serialize};

use crate::schema::bonus;
use crate::schema::dept;
use crate::schema::emp;

#[derive(Debug, Deserialize, Serialize, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = bonus)]
pub struct Bonus {
    pub ename: String,
    pub job: String,
    pub sal: i32,
    pub comm: i32,
}

#[derive(Debug, Deserialize, Serialize, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = dept)]
pub struct Dept {
    pub deptno: i32,
    pub dname: String,
    pub loc: String,
}

#[derive(Debug, Deserialize, Serialize, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = emp)]
pub struct Emp {
    pub empno: i32,
    pub ename: Option<String>,
    pub job: Option<String>,
    pub mgr: Option<i32>,
    pub hiredate: Option<chrono::NaiveDate>,
    pub sal: Option<i32>,
    pub comm: Option<i32>,
    pub deptno: Option<i32>,
    pub tstamp: Option<chrono::NaiveDateTime>,
}
