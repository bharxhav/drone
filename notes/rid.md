GENERAL FORM
============

ri.<service>.<instance>.<resource-type>.<locator>
-> Generic Foundry Resource Identifier structure

ri.<service>>..<resource-type>.<locator>
-> Same structure when the instance segment is empty

COMPASS / FILESYSTEM
====================

ri.compass.main.folder.<UUID>
-> Compass Namespace

ri.compass.main.folder.<UUID>
-> Compass Project

ri.compass.main.folder.<UUID>
-> Compass Folder

NOTE: Namespace, Project, and Folder use the identical RID pattern.
The resourceType metadata is required to distinguish them.

DATASETS / DATA STORAGE
=======================

ri.foundry.main.dataset.<UUID>
-> Foundry Dataset

ri.gps.main.view.<UUID>
-> Restricted View

ri.tables.main.table.<UUID>
-> Virtual Table

ri.mio.main.media-set.<UUID>
-> Media Set

ri.mio.main.media-item.<UUID>
-> Individual Media Set Item

ri.foundry.main.branch.<DATASET-UUID>.<BASE64-BRANCH-NAME>
-> Dataset Branch
-> Example suffix "bWFzdGVy" decodes to "master"

ri.foundry.main.build.<UUID>
-> Dataset/Pipeline Build

ri.foundry.main.job.<UUID>
-> Transform Job belonging to a Build

ri.foundry-sql-server.<INSTANCE>.worksheet.<UUID>
-> Foundry SQL Worksheet

CODE / TRANSFORMS / FUNCTIONS
=============================

ri.stemma.main.repository.<UUID>
-> Code Repository

ri.eddie.main.pipeline.<UUID>
-> Pipeline Builder Pipeline

ri.eddie.main.logic.<UUID>
-> AIP Logic Function / Logic Editor Resource

ri.function-registry.main.function.<UUID>
-> Published Foundry Function

ri.linter.main.recommendation.<UUID>
-> Foundry Linter Recommendation

LANGUAGE MODELS / MACHINE LEARNING / EVALUATIONS
================================================

ri.language-model-service..language-model.<MODEL-SLUG>
-> Language Model Service Model
-> Locator can be a slug rather than a UUID
-> Example locator: anthropic-claude-4-6-sonnet

ri.models.main.model.<UUID>
-> Foundry Machine Learning Model

ri.evals..evaluation-suite.<UUID>
-> AIP Evaluation Suite

ONTOLOGY
========

ri.ontology.main.ontology.<UUID>
-> Ontology

ri.ontology.main.object-type.<UUID>
-> Ontology Object Type

ri.ontology.main.relation.<UUID>
-> Ontology Link Type / Relation

ri.ontology.main.interface.<UUID>
-> Ontology Interface Type

ri.ontology.main.interface-property.<UUID>
-> Interface Property Constraint

ri.ontology.main.type-group.<UUID>
-> Ontology Type Group

ri.actions.main.action-type.<UUID>
-> Ontology Action Type

ri.actions.main.validation-rule.<UUID>
-> Action Type Validation Rule

ri.ontology.main.branch.<UUID>
-> Ontology Branch

ONTOLOGY OBJECT INSTANCES / OBJECT SETS
======================================

ri.phonograph2-objects.<INSTANCE>.object.<UUID>
-> Individual Ontology Object Instance

ri.ontology.main.object.<UUID>
-> Alternate object RID form documented by object-loading APIs

ri.object-set.main.object-set.<UUID>
-> Standard Object Set

ri.object-set.main.versioned-object-set.<UUID>
-> Versioned Object Set

ri.object-set.main.temporary-object-set.<UUID>
-> Temporary Object Set

GLOBAL BRANCHING
================

ri.branch..branch.<UUID>
-> Global Foundry Branch

ri.branch..proposal.<UUID>
-> Global Branch Proposal / Merge Proposal

DATA CONNECTION
===============

ri.magritte..source.<UUID>
-> Data Connection Source

ri.magritte..extract.<UUID>
-> Source Sync / Extract

ri.magritte..agent.<UUID>
-> Data Connection Agent

ri.resource-policy-manager.global.network-egress-policy.<UUID>
-> Network Egress Policy

ri.resource-policy-manager.<INSTANCE>.network-egress-policy.<UUID>
-> Generic environment-specific Network Egress Policy form

ri.connectivityconfig..client-certificate.<LOCATOR>
-> Data Connection Client Certificate

ri.connectivityconfig..server-certificate-bundle.<LOCATOR>
-> Data Connection Server Certificate Bundle

ri.control-panel.main.customer.<UUID>
-> Foundry Enrollment / Customer

ri.time-series-catalog.main.sync.<UUID>
-> Time Series Catalog Sync

WORKSHOP / APPLICATIONS
=======================

ri.workshop.main.module.<UUID>
-> Workshop Module

ri.widgetregistry..widget-set.<UUID>
-> OSDK/Workshop Custom Widget Set

ri.workflow-builder.<INSTANCE>.edit.<UUID>
-> Workflow Builder Resource

ri.slate.<INSTANCE>.document.<UUID>
-> Slate Document

NOTEPAD / CONTOUR
=================

ri.notepad.main.notepad.<UUID>
-> Notepad Document

ri.notepad.<INSTANCE>.notepad-template.<UUID>
-> Notepad Template

ri.contour.<INSTANCE>.analysis.<UUID>
-> Contour Analysis

AUTOMATE / MACHINERY
====================

ri.object-sentinel.main.monitor.<UUID>
-> Automate Automation / Object Set Monitor

ri.machinery.main.document.<UUID>
-> Machinery Business Process Graph

SOLUTION DESIGN
===============

ri.solution-design.main.diagram.<UUID>
-> Solution Design Diagram

ri.solution-design.main.node.group.<UUID>
-> Group Node inside a Solution Design Diagram

ri.solution-design.main.node.object.<UUID>
-> Object Type Node inside a Solution Design Diagram

ri.solution-design.main.node.comment.<UUID>
-> Comment Node inside a Solution Design Diagram

ri.solution-design.main.edge.standard.<UUID>
-> Standard Edge inside a Solution Design Diagram

CIPHER
======

ri.bellaso.main.cipher-channel.<UUID>
-> Cipher Channel

ri.bellaso.main.cipher-license.<UUID>
-> Cipher License

AIP
===

ri.aip-agents.<INSTANCE>.skill.<UUID>
-> AIP Skill

IMPORTANT NON-RID IDENTIFIERS
=============================

<ONTOLOGY-PREFIX>.<KEBAB-CASE-ID>
-> Object Type ID
-> Link Type ID
-> Example: fm2zn78i.qc-run-7e26c0db
-> NOT a RID

<KEBAB-CASE-STRING>
    -> Property Type ID
    -> Example: qc-run-id
    -> NOT a RID

<BARE-UUID>
    -> User ID
    -> Group ID
    -> Marking ID
    -> Action parameter ID in some generated resources
    -> Machinery state or transition ID
    -> Workshop widget/page/section ID
    -> NOT a RID

<UUIDv7>
    -> Execution Trace ID
    -> Sync Run ID in some APIs
    -> NOT necessarily a RID

<SEMANTIC-VERSION>
    -> Function Version
    -> Model/function release version
    -> Example: 1.4.0
    -> NOT part of the function RID

<COMMIT-HASH>
    -> Code Repository Commit
    -> NOT a RID

compass:edit
compass:view
compass:discover
compass:manage
-> Compass Role IDs
-> NOT RIDs
