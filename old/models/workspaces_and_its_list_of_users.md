so a workspace doesnt have only one user who is the owner, it has a lot of users, and each have its own PERMISSIONS
so here is the plan, we will have a table called USERS for a workspace, only the workspace can access it,  the one that owns the list of Users, in this list of users, we can only add existing users from the global database, but in this special list each user has a permission
and for the roles, i guess we will do 3 roles, a guest, who can only read, a normal user who can do what the permissions let him to do, and the admin, who is the owner ofc
now for the kind of permissions, well, the est way to store the permissions, is 
WORKSPACE 
 |_ USERS 
 |  |_ USER_XXXXXXX (thats not the actual format, just a test user)
 |     |_ PERMISSIONS 
 |        |_ TABLE:XXXXX 
 |           |_ u64
 |_ TABLES
    |_ ....

i might be wondering, if i ever forgot smt, why THE HELL am i using an integer, but hey, we're not going to use 
boolean for each permission, do the user can do ts, can the user do that, and loosing 14bit, hohoho nonono, we're using a unsigned 64b integer for the permissions, but then we have to set a strong, scalable method
wait, lemme do the list of the permissions, we might just need a u32 at the end 

wait, how did i forgot, the fuck, each table also has its own permission, not like the user, but like, permissions for other stuff, omg i think im just complicating my self

hmmmmmmmmmmm
i asked chatgpt (yeah i had to deblock this but its for learning purpose, and i hate when i dont understand smtso)

alr so a table has this amount of permissions:

Data
- View rows
- Create rows
- Edit rows
- Delete rows

Schema
- Add columns
- Edit columns
- Delete columns

Views
- Create views
- Edit views
- Delete views

thats 10 in total, so a u32 is clearly enough, but i have to see if there is some other permissions, but for now yeah

oh and also, we might also do some presets, prob when inviting ppl and stuff 

so each user, have a number that represents the permissions the user has for an item in a item, i think we can exlude cells, and just do a field, yeah a col or a row can have diff permissions depending on its owner
 

oh also here is a table made by chatgpt :p (yeah im lazy af)

WORKSPACE
 ├─ users
 │   └─ {user_id}
 │        ├─ role: guest | user | admin
 │        └─ permissions
 │             └─ tables
 │                  └─ {table_id}: u32
 └─ tables
      └─ {table_id}
           └─ metadata...


thats actually what i was thinking abt

alr, so first of all, i have to make for each model, a bitmask for its permission list, and i have to make a WorkspaceUsersService to handle those kind of stuff, and rewrite the WorkspaceService to use WorkspaceUsersService etc, and yeah, i should make a todo list lol

alr, so a workspace user, what it should have, hm, i was thinking, we already know that a guest can only have very restricted read only permissions and a normal user, only have the permissions we gave to him, and the admin ofc have the control over all the workspace, but only the owner have it, and also, well have a permissions table, where we store
PERMISSIONS
 |- tables
 |  |- TableID
 | ...  |_ permissions : u32
 |- bases
 |  |- BaseID
 .. ..  |_ permissions: u32
this permission points to a unique workspace user, and can only modified by the admin
and a workspace user, points to the UserId of the user, and has the created_at modifed_at, it can also have a diff username, the USer that points to it owns the real first/last name, and thats it ig ?  







 
