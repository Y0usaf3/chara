# Permissions System – Implementation TODO

## Workspace

- [ ] Define `WorkspacePermission` bitmask (u32 or u64)
  - Manage users
  - Invite users
  - Remove users
  - Change roles
  - Create bases
  - Delete bases
- [ ] Create presets (guest / user / admin)
- [ ] Store workspace-level permissions per WorkspaceUser
- [ ] Implement checks in `WorkspaceService`
- [ ] Route all workspace actions through permission checks

---

## Base

- [ ] Define `BasePermission` bitmask
  - View base
  - Create tables
  - Delete tables
  - Rename base
  - Manage relations
- [ ] Attach base permissions to WorkspaceUser
- [ ] Implement permission checks in `BaseService`
- [ ] Ensure admin bypass works

---

## Table

- [ ] Define `TablePermission` bitmask (u32)
  - View rows
  - Create rows
  - Edit rows
  - Delete rows
  - Add columns
  - Edit columns
  - Delete columns
  - Create views
  - Edit views
  - Delete views
- [ ] Create permission presets
- [ ] Store table permissions per user
- [ ] Implement checks in `TableService`
- [ ] Deny access when no permission entry exists

---

## Record

- [ ] Decide ownership model (optional)
- [ ] Map actions to table permissions
  - Read → VIEW_ROWS
  - Insert → CREATE_ROWS
  - Update → EDIT_ROWS
  - Delete → DELETE_ROWS
- [ ] Implement checks in `RecordService`
- [ ] Add admin override
- [ ] (Optional) Add owner-based policy layer

---

## Field

- [ ] Map field actions to table permissions
  - Create → ADD_COLUMNS
  - Update → EDIT_COLUMNS
  - Delete → DELETE_COLUMNS
- [ ] Implement checks in `FieldService`
- [ ] Prevent schema mutation without permission

---

## Cell

- [ ] Treat cell edits as record edits
- [ ] Reuse RECORD + TABLE permission checks
- [ ] Implement checks in `CellService`
- [ ] (Optional) Add field-level ownership rules later

---

## View

- [ ] Define `ViewPermission` bitmask (or reuse table bits)
  - Create view
  - Edit view
  - Delete view
- [ ] Store permissions per table
- [ ] Implement checks in `ViewService`
- [ ] Prevent view mutation for read-only users

---

## Relation

- [ ] Map relation actions to base/table permissions
  - Create relation
  - Delete relation
- [ ] Implement checks in `RelationService`
- [ ] Ensure schema-level permission required

---

## User (WorkspaceUser)

- [ ] Create `WorkspaceUser` model
  - user_id
  - role
  - workspace permissions
  - base permissions
  - table permissions
- [ ] Implement add/remove user logic
- [ ] Implement role change logic
- [ ] Apply permission presets on invite

---

## Permission Infrastructure

- [ ] Centralize all bitmask definitions
- [ ] Implement helper functions:
  - has_permission()
  - grant_permission()
  - revoke_permission()
- [ ] Implement admin short-circuit
- [ ] Ensure permissions are serializable
- [ ] Add tests for each permission type

---

## Services Refactor

- [ ] Create `WorkspaceUsersService`
- [ ] Move all permission logic into it
- [ ] Refactor all other services to call it
- [ ] Remove duplicated permission checks

---

## Tests

- [ ] Guest access tests
- [ ] User partial permission tests
- [ ] Admin bypass tests
- [ ] Missing permission = deny
- [ ] Preset correctness tests

