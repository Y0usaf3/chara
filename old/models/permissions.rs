use crate::bitmask_serde;

bitmask! {
    pub mask CellPermissions: u32 where flags CellPermission {
        View       = 0b0000_0000_0001,
        Edit       = 0b0000_0000_0010,
        Delete     = 0b0000_0000_0100,
        AddComment = 0b0000_0000_1000,
    }
}
bitmask_serde!(CellPermissions);

// --- Field Permissions ---
bitmask! {
    pub mask FieldPermissions: u32 where flags FieldPermission {
        View       = 0b0000_0000_0001,
        Edit       = 0b0000_0000_0010,
        Delete     = 0b0000_0000_0100,
        AddFormula = 0b0000_0000_1000,
    }
}
bitmask_serde!(FieldPermissions);

// --- Record Permissions ---
bitmask! {
    pub mask RecordPermissions: u32 where flags RecordPermission {
        View       = 0b0000_0000_0001,
        Edit       = 0b0000_0000_0010,
        Delete     = 0b0000_0000_0100,
        Archive    = 0b0000_0000_1000,
    }
}
bitmask_serde!(RecordPermissions);

// --- Relation Permissions ---
bitmask! {
    pub mask RelationPermissions: u32 where flags RelationPermission {
        View       = 0b0000_0000_0001,
        Edit       = 0b0000_0000_0010,
        Delete     = 0b0000_0000_0100,
        Link       = 0b0000_0000_1000,
    }
}
bitmask_serde!(RelationPermissions);

// --- Table Permissions --- (already defined)
bitmask! {
    pub mask TablePermissions: u32 where flags TablePermission {
        ViewRows       = 0b0000_0000_0001,
        CreateRows     = 0b0000_0000_0010,
        EditRows       = 0b0000_0000_0100,
        DeleteRows     = 0b0000_0000_1000,
        AddColumns     = 0b0000_0001_0000,
        EditColumns    = 0b0000_0010_0000,
        DeleteColumns  = 0b0000_0100_0000,
        CreateViews    = 0b0000_1000_0000,
        EditViews      = 0b0001_0000_0000,
        DeleteViews    = 0b0010_0000_0000,
    }
}
bitmask_serde!(TablePermissions);

// --- Workspace Permissions ---
bitmask! {
    pub mask WorkspacePermissions: u32 where flags WorkspacePermission {
        View       = 0b0000_0000_0001,
        Edit       = 0b0000_0000_0010,
        Delete     = 0b0000_0000_0100,
        ManageUsers= 0b0000_0000_1000,
        ManageApps = 0b0000_0001_0000,
    }
}
bitmask_serde!(WorkspacePermissions);

// --- Workspace Users Permissions ---
bitmask! {
    pub mask WorkspaceUsersPermissions: u32 where flags WorkspaceUsersPermission {
        Invite      = 0b0000_0000_0001,
        Remove      = 0b0000_0000_0010,
        Promote     = 0b0000_0000_0100,
        Demote      = 0b0000_0000_1000,
    }
}
bitmask_serde!(WorkspaceUsersPermissions);

bitmask! {
    pub mask BasePermissions: u32 where flags BasePermission {
        View          = 0b0000_0000_0001,
        Edit          = 0b0000_0000_0010,
        Delete        = 0b0000_0000_0100,
        ManageTables  = 0b0000_0000_1000,
        ManageViews   = 0b0000_0001_0000,
        ManageUsers   = 0b0000_0010_0000,
        ManageWorkspaces = 0b0000_0100_0000,
    }
}
bitmask_serde!(BasePermissions);
