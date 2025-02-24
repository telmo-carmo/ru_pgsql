// @generated automatically by Diesel CLI.

diesel::table! {
    bonus (ename) {
        ename -> Text,
        job -> Text,
        sal -> Integer,
        comm -> Integer,
    }
}

diesel::table! {
    dept (deptno) {
        deptno -> Integer,
        dname -> Text,
        loc -> Text,
    }
}

diesel::table! {
    emp (empno) {
        empno -> Integer,
        ename -> Nullable<Text>,
        job -> Nullable<Text>,
        mgr -> Nullable<Integer>,
        hiredate -> Nullable<Date>,
        sal -> Nullable<Integer>,
        comm -> Nullable<Integer>,
        deptno -> Nullable<Integer>,
        tstamp -> Nullable<Timestamp>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(bonus, dept, emp);
