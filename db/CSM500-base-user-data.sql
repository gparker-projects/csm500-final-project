-- -------------------------------------------------------------------------------------
-- Loads seed USERS data for the system. We actually need to control the id for this table, in order to 
-- properly assign permissions.
--
-- view using: SELECT id, name, username, email, created_at, password FROM users order by id;
-- -------------------------------------------------------------------------------------
INSERT INTO users (id, username, name, email, created_at, password) OVERRIDING SYSTEM VALUE VALUES (1, 'ghouse',   'Dr. Gregory House', 'ghouse@google.com',   '2026-06-06 12:00:00', 'doctor');
INSERT INTO users (id, username, name, email, created_at, password) OVERRIDING SYSTEM VALUE VALUES (2, 'chath',    'Carol Hathaway',    'chath@google.com',    '2026-06-06 12:00:00', 'nurse1');
INSERT INTO users (id, username, name, email, created_at, password) OVERRIDING SYSTEM VALUE VALUES (3, 'sspants',  'Spongebob Squarepants',       'spants@google@google.com',  '2026-06-06 12:00:00', 'csm500');
INSERT INTO users (id, username, name, email, created_at, password) OVERRIDING SYSTEM VALUE VALUES (4, 'jwilson',  'James Wilson',      'jwilson@google.com',  '2026-06-06 12:00:00', 'labtech');
INSERT INTO users (id, username, name, email, created_at, password) OVERRIDING SYSTEM VALUE VALUES (5, 'lcuddy',   'Lisa Cuddy',        'lcuddy@google.com',   '2026-06-06 12:00:00', 'pharm');
INSERT INTO users (id, username, name, email, created_at, password) OVERRIDING SYSTEM VALUE VALUES (6, 'rchase',   'Robert Chase',      'rchase@google.com',   '2026-06-06 12:00:00', 'housek');
INSERT INTO users (id, username, name, email, created_at, password) OVERRIDING SYSTEM VALUE VALUES (7, 'eforeman', 'Eric Foreman',      'eforeman@google.com', '2026-06-06 12:00:00', 'admin');
INSERT INTO users (id, username, name, email, created_at, password) OVERRIDING SYSTEM VALUE VALUES (8, 'mcrabs',   'Mister Crabs',      'mcrabs@google.ca', '2026-06-06 12:00:00', 'csm500');
COMMIT;

-- -------------------------------------------------------------------------------------
-- Loads seed USER_PERMISSION data for the system.
--
-- view using: SELECT id, name, expiry_datetime FROM permission;
-- -------------------------------------------------------------------------------------

-- User 3 will have full permissions to everything, for the initial draft implementation
--
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id) VALUES ('Y', 3, 4, 2, 1);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id) VALUES ('Y', 3, 4, 2, 2);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id) VALUES ('Y', 3, 4, 2, 3);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id) VALUES ('Y', 3, 4, 2, 4);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id) VALUES ('Y', 3, 4, 2, 5);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id) VALUES ('Y', 3, 4, 2, 6);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id) VALUES ('Y', 3, 4, 2, 7);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id) VALUES ('Y', 3, 4, 2, 8);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id) VALUES ('Y', 3, 4, 2, 9);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id) VALUES ('Y', 3, 4, 2, 10);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id) VALUES ('Y', 3, 4, 2, 11);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id) VALUES ('Y', 3, 4, 2, 12);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id) VALUES ('Y', 3, 4, 2, 13);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id) VALUES ('Y', 3, 4, 2, 14);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id) VALUES ('Y', 3, 4, 2, 15);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id) VALUES ('Y', 3, 4, 2, 16);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id) VALUES ('Y', 3, 4, 2, 17);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id) VALUES ('Y', 3, 4, 2, 19);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id) VALUES ('Y', 3, 4, 2, 20);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id) VALUES ('Y', 3, 4, 2, 21);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id) VALUES ('Y', 3, 4, 2, 22);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id) VALUES ('Y', 3, 4, 2, 23);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id) VALUES ('Y', 3, 4, 2, 24);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id) VALUES ('Y', 3, 4, 2, 25);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id) VALUES ('Y', 3, 4, 2, 26);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id) VALUES ('Y', 3, 4, 2, 27);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id) VALUES ('Y', 3, 4, 2, 28);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id) VALUES ('Y', 3, 4, 2, 29);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id) VALUES ('Y', 3, 4, 2, 30);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id) VALUES ('Y', 3, 4, 2, 31);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id) VALUES ('Y', 3, 4, 2, 32);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id) VALUES ('Y', 3, 4, 2, 33);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id) VALUES ('Y', 3, 4, 2, 34);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id) VALUES ('Y', 3, 4, 2, 35);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id) VALUES ('Y', 3, 4, 2, 36);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id) VALUES ('Y', 3, 4, 2, 37);

-- user 3 will also have a few permissions for another department: should not show as a duplicate
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id) VALUES ('Y', 3, 1, 2, 1);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id) VALUES ('Y', 3, 1, 2, 2);

-- user 3 will also have a few permissions that are not active, in the other department; should not show at all
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id) VALUES ('N', 3, 1, 2, 3);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id) VALUES ('N', 3, 1, 2, 4);

COMMIT;