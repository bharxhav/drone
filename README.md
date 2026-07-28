<p align="center">
  <img src="assets/logo.svg" width="144" alt="drone logo">
</p>

<h1 align="center">drone</h1>

<p align="center">a cli for palantir foundry</p>

<br>
<br>

**synopsis**

```sh
drone [--help | --version]
drone <verb> <noun> [identifier] [options]
drone context [show | set <key> <value>]
drone man [domain] [scope...]
```

<br>
<br>

---

## discovery

<br>

### man

browse palantir foundry documentation.

```sh
drone man [--json]
drone man product <scope...>
drone man platform <scope...>
drone man updates <scope...>
```

<br>

### list

enumerate resources. paginated.

```
# who is in this enrollment?
list users
list groups
list current-groups

# what can they access?
list hosts
list auth-providers

# how is access governed?
list markings
list marking-categories
list marking-members <markingId>
list marking-roles <markingId>
list enrollment-roles <enrollmentRid>
list organization-roles <orgId>
list org-available-roles <orgId>

# who is in this group?
list group-members <groupId>
list group-memberships <groupId>

# what is in the filesystem?
list spaces
list children <folderRid>
list resource-roles <rid>
list resource-markings <rid>

# what does this project contain?
list project-refs <projectRid>
list project-orgs <projectRid>

# what is in this dataset?
list branches <datasetRid>
list dataset-files <datasetRid>
list dataset-transactions <datasetRid>

# what connections exist?
list file-imports
list table-imports

# what does the ontology expose?
list ontologies
list object-types <ontologyRid>
list link-types <ontologyRid> <objectType>
list action-types <ontologyRid>
list query-types <ontologyRid>
list interface-types <ontologyRid>
list ontology-value-types <ontologyRid>

# what objects are in it?
list objects <ontologyRid> <objectType>
list linked-objects <ontologyRid> <type> <pk> <linkType>
list attachments <ontologyRid> <type> <pk> <property>

# what is scheduled?
list schedules
list schedule-runs <scheduleRid>
list build-jobs <buildRid>

# what agents exist?
list sessions <agentRid>
list agent-sessions <agentRid>
list agent-versions <agentRid>

# what models are deployed?
list model-versions <modelRid>
list model-studio-configs <studioRid>
list model-studio-runs <studioRid>
list model-studio-trainers <studioRid>
list live-deployments

# what apps are deployed?
list app-versions <websiteRid>

# what is being observed?
list scan-records
list check-reports <checkRid>
list latest-check-reports <checkRid>
list executions <resourceRid>

# what happened?
list log-files
```

<br>

### search

query resources by criteria.

```
search users

search groups

search objects <ontologyRid> <objectType>

search experiments

search records <checkpointRid>
```

<br>
<br>

---

## inspection

<br>

### get

retrieve a single resource by identifier.

```
# identity
get user <id>
get user-markings <userId>
get user-picture <userId>
get user-provider-info <userId>
get current-user

get group <id>
get group-provider-info <groupId>
get group-expiration-policy <groupId>

get enrollment <id>
get current-enrollment
get organization <id>

get auth-provider <id>

# governance
get marking <id>
get marking-category <id>
get cbac-banner <enrollmentRid>
get cbac-marking-restrictions <markingId>

# filesystem
get resource <rid>
get resource-by-path <path>
get access-requirements <rid>
get folder <rid>
get project <rid>

# datasets
get dataset <rid>
get dataset-schema <rid>
get dataset-health-checks <rid>
get dataset-health-reports <rid>
get dataset-jobs <rid>
get dataset-schedules <rid>
get branch <datasetRid> <branchId>
get branch-history <datasetRid> <branchId>
get transaction <datasetRid> <transactionRid>
get dataset-file <datasetRid> <filePath>
get dataset-file-content <datasetRid> <filePath>
get view <datasetRid> <viewRid>

# connectivity
get connection <rid>
get connection-config <rid>
get file-import <rid>
get table-import <rid>

# ontology schema
get ontology <rid>
get ontology-metadata <rid>
get object-type <ontologyRid> <objectType>
get object-type-metadata <ontologyRid> <objectType>
get object-type-history <ontologyRid> <objectType>
get link-type <ontologyRid> <objectType> <linkType>
get action-type <ontologyRid> <actionType>
get query-type <ontologyRid> <queryType>
get interface-type <ontologyRid> <interfaceType>
get ontology-value-type <ontologyRid> <valueType>

# ontology objects
get object <ontologyRid> <objectType> <primaryKey>
get linked-object <ontologyRid> <type> <pk> <linkType>

# ontology attachments and properties
get attachment <rid>
get attachment-content <rid>
get attachment-property <ontologyRid> <type> <pk> <property>
get attachment-property-content <ontologyRid> <type> <pk> <property>

# time series
get timeseries-first <ontologyRid> <type> <pk> <property>
get timeseries-last <ontologyRid> <type> <pk> <property>
get timeseries-latest <ontologyRid> <type> <pk> <property>

# orchestration
get schedule <rid>
get schedule-version <rid>
get schedule-resources <rid>
get build <rid>
get job <rid>

# aip agents
get agent <rid>
get agent-version <agentRid> <versionRid>
get session <rid>
get session-trace <sessionRid> <traceId>
get session-content <sessionRid>
get rag-context <sessionRid>

# media sets
get media-set <rid>
get media-item-info <mediaSetRid> <itemRid>
get media-item-metadata <mediaSetRid> <itemRid>
get media-item-reference <mediaSetRid> <itemRid>
get media-item-by-path <mediaSetRid> <path>
get media-transform-status <mediaSetRid> <jobId>
get media-transform-result <mediaSetRid> <jobId>

# models
get model <rid>
get model-version <modelRid> <versionRid>
get model-function <modelRid> <functionRid>
get model-studio <rid>
get model-studio-config <studioRid> <versionRid>
get model-studio-trainer <studioRid> <trainerRid>
get experiment <rid>
get live-deployment <rid>

# data health
get check <rid>
get check-report <checkRid> <reportRid>

# notepad
get export-job <rid>
get generation-job <rid>

# third party apps
get website <rid>
get app-version <rid>

# ontology mcp
get mcp-server <rid>

# sql queries
get sql-status <queryId>
get sql-results <queryId>

# streams
get stream <rid>
get stream-offsets <rid>

# checkpoints
get record <checkpointRid> <recordId>

# audit
get log-file-content <logFileRid>
```

<br>

### get --batch

retrieve multiple resources in one call.

```
get --batch users <id>...
get --batch groups <id>...
get --batch markings <id>...

get --batch resources <rid>...
get --batch resources-by-path <path>...
get --batch folders <rid>...

get --batch dataset-schemas <rid>...

get --batch connection-configs <rid>...

get --batch object-types <rid>...
get --batch action-types <rid>...
get --batch link-types <rid>...

get --batch schedules <rid>...
get --batch builds <rid>...
get --batch jobs <rid>...

get --batch records <checkpointRid> <id>...

get --batch queries <rid>...
```

<br>

### read

download binary content or rendered data.

```
read dataset-table <datasetRid>

read media-content <ontologyRid> <type> <pk> <property>

read media-item <mediaSetRid> <itemRid>
read media-item --original <mediaSetRid> <itemRid>

read notepad-file <rid>
```

<br>

### stream

consume time series or value bank data points.

```
stream timeseries <ontologyRid> <type> <pk> <property>

stream values <ontologyRid> <type> <pk> <property>
```

<br>

### aggregate

compute aggregations over ontology data.

```
aggregate objects <ontologyRid> <objectType>

aggregate object-set <ontologyRid>
```

<br>

### load

materialize an object set into rows.

```
load object-set <ontologyRid>
load object-set --multi-type <ontologyRid>
load object-set --interfaces <ontologyRid>
```

<br>

### decrypt

decrypt an encrypted property value.

```
decrypt property <ontologyRid> <type> <pk> <property>
```

<br>

### export

retrieve experiment artifacts.

```
export artifact-table --json <experimentRid>
export artifact-table --parquet <experimentRid>

export experiment-series --json <experimentRid>
export experiment-series --parquet <experimentRid>
```

<br>

### latest

retrieve the most recent version.

```
latest model-studio-config <studioRid>
```

<br>
<br>

---

## mutation

<br>

### create

bring a new resource into existence.

```
# identity and governance
create user --preregister <authProviderId>
create group
create group --preregister <authProviderId>
create organization
create marking
create marking-category

# filesystem
create folder
create project
create project --from-template <templateRid>

# datasets
create dataset
create branch <datasetRid>
create transaction <datasetRid>
create view <datasetRid>

# connectivity
create connection
create file-import
create table-import
create virtual-table

# ontology
create object-set <ontologyRid>

# orchestration
create schedule
create build

# aip agents
create session <agentRid>

# streams
create streaming-dataset
create stream <datasetRid>

# media
create media-transaction <mediaSetRid>

# models
create model
create model-function <modelRid>
create model-studio
create model-studio-config <studioRid>
create live-deployment

# data health
create check

# notepad
create export-job
```

<br>

### replace

overwrite a mutable resource entirely.

```
replace group <id>
replace user-provider-info <userId>
replace group-provider-info <groupId>
replace group-expiration-policy <groupId>
replace organization <id>
replace marking <id>
replace marking-category <id>

replace file-import <rid>
replace table-import <rid>

replace schedule <rid>

replace live-deployment <rid>
replace model-function <modelRid> <functionRid>

replace check <rid>

replace backing-datasets <datasetRid> <viewRid>
```

<br>

### update

patch specific fields without full replacement.

```
update connection-secrets <connectionRid>
update connection-exports <connectionRid>

update session-title <sessionRid>
```

<br>

### put

upsert: create-or-replace content.

```
put dataset-schema <datasetRid>

put media-item <mediaSetRid>
```

<br>

### delete

remove a resource.

```
delete user <id>
delete group <id>

delete resource <rid>
delete resource --permanent <rid>

delete branch <datasetRid> <branchId>
delete dataset-file <datasetRid> <filePath>

delete file-import <rid>
delete table-import <rid>

delete schedule <rid>

delete session <sessionRid>

delete check <rid>

delete app-version <versionRid>
```

<br>

### restore

recover a soft-deleted resource.

```
restore resource <rid>
```

<br>

### add

attach members, roles, or associations.

```
add group-members <groupId> <principalId>...

add marking-members <markingId> <principalId>...
add marking-roles <markingId>

add enrollment-roles <enrollmentRid>
add organization-roles <organizationRid>

add resource-roles <rid>
add resource-markings <rid> <markingId>...

add project-orgs <projectRid> <organizationRid>...
add project-refs <projectRid> <resourceRid>...

add backing-datasets <datasetRid> <viewRid>
add primary-key <datasetRid> <viewRid>

add board-objects <boardRid>
```

<br>

### remove

detach members, roles, or associations.

```
remove group-members <groupId> <principalId>...

remove marking-members <markingId> <principalId>...
remove marking-roles <markingId>

remove enrollment-roles <enrollmentRid>
remove organization-roles <organizationRid>

remove resource-roles <rid>
remove resource-markings <rid> <markingId>...

remove project-orgs <projectRid> <organizationRid>...
remove project-refs <projectRid> <resourceRid>...

remove backing-datasets <datasetRid> <viewRid>
```

<br>

### upload

transfer binary content into foundry.

```
upload dataset-file <datasetRid> <filePath>

upload attachment

upload media-content <ontologyRid> <type> <pk> <property>

upload media <mediaSetRid>

upload app-version <websiteRid>

upload jdbc-drivers <connectionRid>
```

<br>

### publish

write records into a stream.

```
publish record <streamRid>

publish records <streamRid>

publish record --binary <streamRid>
```

<br>

### register

register external content by reference.

```
register media-item <mediaSetRid>
```

<br>

### move

reposition resources.

```
move board-objects <boardRid>
```

<br>
<br>

---

## execution

<br>

### execute

run a computation or import job.

```
execute query <ontologyRid> <queryType>

execute function <functionRid>
execute function --stream <functionRid>

execute sql
execute ontology-sql

execute file-import <fileImportRid>
execute table-import <tableImportRid>
```

<br>

### apply

trigger an ontology action.

```
apply action <ontologyRid> <actionType>
apply action --batch <ontologyRid> <actionType>
```

<br>

### run

kick off a scheduled or on-demand process.

```
run schedule <scheduleRid>
```

<br>

### launch

start a long-running interactive session.

```
launch model-studio <studioRid>
```

<br>

### generate

trigger document generation from a template.

```
generate template <templateRid>
```

<br>

### transform

apply a transformation to content.

```
transform live-deployment <deploymentRid>

transform media-item <mediaSetRid> <itemRid>
```

<br>

### render

produce visual output.

```
render invocation-object <objectRid>

render symbol <symbolName>
```

<br>

### continue

resume an in-progress agent conversation.

```
continue session <sessionRid>
continue session --stream <sessionRid>
```

<br>

### proxy

pass-through to hosted llm apis.

```
proxy anthropic <modelId>

proxy openai-chat <modelId>

proxy openai-responses <modelId>

proxy openai-embeddings <modelId>
```

<br>
<br>

---

## lifecycle

<br>

### commit

finalize a transaction.

```
commit transaction <datasetRid> <transactionRid>

commit media-transaction <mediaSetRid> <transactionRid>
```

<br>

### abort

abandon a transaction.

```
abort transaction <datasetRid> <transactionRid>

abort media-transaction <mediaSetRid> <transactionRid>
```

<br>

### cancel

stop an in-progress operation.

```
cancel build <buildRid>

cancel session <sessionRid>

cancel sql <queryId>
```

<br>

### pause

suspend a schedule.

```
pause schedule <scheduleRid>
```

<br>

### unpause

resume a suspended schedule.

```
unpause schedule <scheduleRid>
```

<br>

### deploy

activate a website version.

```
deploy website <websiteRid> <versionRid>
```

<br>

### undeploy

deactivate a website.

```
undeploy website <websiteRid>
```

<br>

### promote

advance a version to a higher stage.

```
promote model-version <modelRid> <versionRid>
```

<br>

### reset

return something to its initial state.

```
reset stream <streamRid>
```

<br>

### revoke

invalidate credentials.

```
revoke tokens <userId>
```

<br>

### save

persist a transient result.

```
save generation-document <generationJobRid>
```

<br>

### clear

remove content at a location.

```
clear media-item <mediaSetRid> <path>
```

<br>

### parse

interpret a structured string.

```
parse classifications
```

<br>
<br>

---

## context

stateful defaults. persisted across invocations.

```sh
drone context show
drone context set hostname <url>
drone context set token <token>
drone context set project <rid>
drone context set dataset <rid>
drone context set ontology <rid>
drone context set agent <rid>
drone context set output [json | table | raw]
```

<br>
<br>
