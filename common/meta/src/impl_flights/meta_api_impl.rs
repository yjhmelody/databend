// Copyright 2020 Datafuse Labs.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

use common_metatypes::MetaId;
use common_metatypes::MetaVersion;
use common_planners::CreateDatabasePlan;
use common_planners::CreateTablePlan;
use common_planners::DropDatabasePlan;
use common_planners::DropTablePlan;

use crate::action_declare;
use crate::meta_api::MetaApi;
use crate::store_do_action::StoreDoAction;
use crate::vo::CreateDatabaseReply;
use crate::vo::CreateTableReply;
use crate::vo::DatabaseInfo;
use crate::vo::GetDatabasesReply;
use crate::vo::GetTablesReply;
use crate::vo::TableInfo;
use crate::RequestFor;
use crate::StoreClient;

#[async_trait::async_trait]
impl MetaApi for StoreClient {
    /// Create database call.
    async fn create_database(
        &self,
        plan: CreateDatabasePlan,
    ) -> common_exception::Result<CreateDatabaseReply> {
        self.do_action(CreateDatabaseAction { plan }).await
    }

    async fn get_database(&self, db: &str) -> common_exception::Result<DatabaseInfo> {
        self.do_action(GetDatabaseAction { db: db.to_string() })
            .await
    }

    /// Drop database call.
    async fn drop_database(&self, plan: DropDatabasePlan) -> common_exception::Result<()> {
        self.do_action(DropDatabaseAction { plan }).await
    }

    /// Create table call.
    async fn create_table(
        &self,
        plan: CreateTablePlan,
    ) -> common_exception::Result<CreateTableReply> {
        self.do_action(CreateTableAction { plan }).await
    }

    /// Drop table call.
    async fn drop_table(&self, plan: DropTablePlan) -> common_exception::Result<()> {
        self.do_action(DropTableAction { plan }).await
    }

    /// Get table.
    async fn get_table(&self, db: &str, table: &str) -> common_exception::Result<TableInfo> {
        self.do_action(GetTableAction {
            db: db.to_string(),
            table: table.to_string(),
        })
        .await
    }

    async fn get_table_by_id(
        &self,
        tbl_id: MetaId,
        tbl_ver: Option<MetaVersion>,
    ) -> common_exception::Result<TableInfo> {
        self.do_action(GetTableExtReq { tbl_id, tbl_ver }).await
    }

    async fn get_databases(&self) -> common_exception::Result<GetDatabasesReply> {
        self.do_action(GetDatabasesAction {}).await
    }

    /// Get tables.
    async fn get_tables(&self, db: &str) -> common_exception::Result<GetTablesReply> {
        self.do_action(GetTablesAction { db: db.to_string() }).await
    }
}

// == database actions ==
// - create database
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct CreateDatabaseAction {
    pub plan: CreateDatabasePlan,
}
action_declare!(
    CreateDatabaseAction,
    CreateDatabaseReply,
    StoreDoAction::CreateDatabase
);

// - get database
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct GetDatabaseAction {
    pub db: String,
}
action_declare!(GetDatabaseAction, DatabaseInfo, StoreDoAction::GetDatabase);

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct DropDatabaseAction {
    pub plan: DropDatabasePlan,
}
action_declare!(DropDatabaseAction, (), StoreDoAction::DropDatabase);

// == table actions ==
// - create table
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct CreateTableAction {
    pub plan: CreateTablePlan,
}
action_declare!(
    CreateTableAction,
    CreateTableReply,
    StoreDoAction::CreateTable
);

// - drop table
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct DropTableAction {
    pub plan: DropTablePlan,
}
action_declare!(DropTableAction, (), StoreDoAction::DropTable);

// - get table
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct GetTableAction {
    pub db: String,
    pub table: String,
}

action_declare!(GetTableAction, TableInfo, StoreDoAction::GetTable);

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct GetTableExtReq {
    pub tbl_id: MetaId,
    pub tbl_ver: Option<MetaVersion>,
}
action_declare!(GetTableExtReq, TableInfo, StoreDoAction::GetTableExt);

// - get tables
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct GetTablesAction {
    pub db: String,
}

action_declare!(GetTablesAction, GetTablesReply, StoreDoAction::GetTables);

// -get databases

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq)]
pub struct GetDatabasesAction;

action_declare!(
    GetDatabasesAction,
    GetDatabasesReply,
    StoreDoAction::GetDatabases
);