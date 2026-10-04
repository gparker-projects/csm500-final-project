-- -------------------------------------------------------------------------------------
-- Loads seed USERS data for the system. We actually need to control the id for this table, in order to 
-- properly assign permissions.
--
-- view using: SELECT id, name, username, email, created_at, password FROM users order by id;
-- -------------------------------------------------------------------------------------
INSERT INTO users (id, username, name, email, created_timestamp, password) OVERRIDING SYSTEM VALUE VALUES (1, 'ghouse',   'Dr. Gregory House', 'ghouse@google.com',   '2026-06-06 12:00:00', 'doctor');
INSERT INTO users (id, username, name, email, created_timestamp, password) OVERRIDING SYSTEM VALUE VALUES (2, 'chath',    'Carol Hathaway',    'chath@google.com',    '2026-06-06 12:00:00', 'nurse1');
INSERT INTO users (id, username, name, email, created_timestamp, password) OVERRIDING SYSTEM VALUE VALUES (3, 'mma1',     'Mary Medical AssistantOne',       'mma1@google@google.com',  '2026-06-06 12:00:00', 'csm500');
INSERT INTO users (id, username, name, email, created_timestamp, password) OVERRIDING SYSTEM VALUE VALUES (4, 'mma2',     'Mattie Medical AssistantTwo',      'mma2@google.com',  '2026-06-06 12:00:00', 'csm500');
INSERT INTO users (id, username, name, email, created_timestamp, password) OVERRIDING SYSTEM VALUE VALUES (5, 'lcuddy',   'Lisa Cuddy',        'lcuddy@google.com',   '2026-06-06 12:00:00', 'admin');
INSERT INTO users (id, username, name, email, created_timestamp, password) OVERRIDING SYSTEM VALUE VALUES (6, 'rchase',   'Dr. Robert Chase',      'rchase@google.com',   '2026-06-06 12:00:00', 'csm500');
INSERT INTO users (id, username, name, email, created_timestamp, password) OVERRIDING SYSTEM VALUE VALUES (7, 'dramoray', 'Dr. Drake Ramoray',      'dramoray@google.com', '2026-06-06 12:00:00', 'joey');
INSERT INTO users (id, username, name, email, created_timestamp, password) OVERRIDING SYSTEM VALUE VALUES (8, 'pporter',  'Peter Porter',      'pporter@google.ca', '2026-06-06 12:00:00', 'csm500');
INSERT INTO users (id, username, name, email, created_timestamp, password) OVERRIDING SYSTEM VALUE VALUES (9, 'invalid',  'Ian Valid',      'invalid@google.ca', '2026-10-03 12:00:00', 'csm500');
COMMIT;

-- -------------------------------------------------------------------------------------
-- Loads seed USER_PERMISSION data for the system.
--
-- view using: SELECT id, name, expiry_datetime FROM permission;
-- -------------------------------------------------------------------------------------

-- Dr. Gregory House will have the Physician role, with access to clinical data mostly
--
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id, site_id) VALUES ('Y', 1, 11, 3, 1, 1);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id, site_id) VALUES ('Y', 1, 11, 3, 2, 1);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id, site_id) VALUES ('Y', 1, 11, 3, 6, 1);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id, site_id) VALUES ('Y', 1, 11, 3, 8, 1);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id, site_id) VALUES ('Y', 1, 11, 3, 9, 1);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id, site_id) VALUES ('Y', 1, 11, 3, 10, 1);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id, site_id) VALUES ('Y', 1, 11, 3, 11, 1);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id, site_id) VALUES ('Y', 1, 11, 3, 12, 1);

COMMIT;

-- Carol Hathaway is a nurse, now with partial permissions
--
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id, site_id) VALUES ('Y', 2, 4, 2, 1, 1);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id, site_id) VALUES ('Y', 2, 4, 2, 2, 1);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id, site_id) VALUES ('Y', 2, 4, 2, 3, 1);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id, site_id) VALUES ('Y', 2, 4, 2, 6, 1);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id, site_id) VALUES ('Y', 2, 4, 2, 7, 1);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id, site_id) VALUES ('Y', 2, 4, 2, 8, 1);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id, site_id) VALUES ('Y', 2, 4, 2, 9, 1);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id, site_id) VALUES ('Y', 2, 4, 2, 10, 1);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id, site_id) VALUES ('Y', 2, 4, 2, 11, 1);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id, site_id) VALUES ('Y', 2, 4, 2, 12, 1);

-- user 3 will also have a few permissions for another department: should not show as a duplicate
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id, site_id) VALUES ('Y', 2, 1, 2, 1, 1);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id, site_id) VALUES ('Y', 2, 1, 2, 2, 1);

-- user 3 will also have a few permissions that are not active, in the other department; should not show at all
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id, site_id) VALUES ('N', 2, 1, 2, 3, 1);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id, site_id) VALUES ('N', 2, 1, 2, 4, 1);

COMMIT;

-- Mary Medical AssistantOne will have the Medical Assistant role, with access to non-clinical data only 
--
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id, site_id) VALUES ('Y', 3, 1, 1, 1, 1);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id, site_id) VALUES ('Y', 3, 1, 1, 3, 1);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id, site_id) VALUES ('Y', 3, 1, 1, 4, 1);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id, site_id) VALUES ('Y', 3, 1, 1, 5, 1);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id, site_id) VALUES ('Y', 3, 1, 1, 7, 1);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id, site_id) VALUES ('Y', 3, 1, 1, 8, 1);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id, site_id) VALUES ('Y', 3, 1, 1, 11, 1);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id, site_id) VALUES ('Y', 3, 1, 1, 12, 1);

insert into user_permission (active_flag,users_id, department_id, role_id, permission_id, site_id)
                      values('Y', 3, 1, 1, 100006, 1); -- PATIENT TRANSFER
                      
insert into user_permission (active_flag,users_id, department_id, role_id, permission_id, site_id)
                      values('Y', 3, 1, 1, 100040, 1); -- ALERTS
                      
insert into user_permission (active_flag,users_id, department_id, role_id, permission_id, site_id)
                      values('Y', 3, 1, 1, 100041, 1); -- APPOINTMENTS
          
          insert into user_permission (active_flag,users_id, department_id, role_id, permission_id, site_id)
                      values('Y', 3, 1, 1, 100046, 1); -- DOCUMENTS
                      
          insert into user_permission (active_flag,users_id, department_id, role_id, permission_id, site_id)
                      values('Y', 3, 1, 1, 100052, 1); -- medical insurance
                      
          insert into user_permission (active_flag,users_id, department_id, role_id, permission_id, site_id)
                      values('Y', 3, 1, 1, 100049, 1); -- next of kin
                      
          insert into user_permission (active_flag,users_id, department_id, role_id, permission_id, site_id)
                      values('Y', 3, 1, 1, 100050, 1); -- primary address
COMMIT;

-- Mary Medical AssistantTwo will have the Medical Assistant role, with access to non-clinical data only
--
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id, site_id) VALUES ('Y', 4, 1, 1, 1, 1);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id, site_id) VALUES ('Y', 4, 1, 1, 3, 1);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id, site_id) VALUES ('Y', 4, 1, 1, 4, 1);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id, site_id) VALUES ('Y', 4, 1, 1, 5, 1);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id, site_id) VALUES ('Y', 4, 1, 1, 7, 1);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id, site_id) VALUES ('Y', 4, 1, 1, 8, 1);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id, site_id) VALUES ('Y', 4, 1, 1, 11, 1);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id, site_id) VALUES ('Y', 4, 1, 1, 12, 1);

COMMIT;

-- Lisa Cuddy will have the Admin role, with access to everything
--
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id, site_id) VALUES ('Y', 5, 4, 6, 1, 1);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id, site_id) VALUES ('Y', 5, 4, 6, 2, 1);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id, site_id) VALUES ('Y', 5, 4, 6, 3, 1);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id, site_id) VALUES ('Y', 5, 4, 6, 4, 1);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id, site_id) VALUES ('Y', 5, 4, 6, 5, 1);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id, site_id) VALUES ('Y', 5, 4, 6, 6, 1);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id, site_id) VALUES ('Y', 5, 4, 6, 7, 1);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id, site_id) VALUES ('Y', 5, 4, 6, 8, 1);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id, site_id) VALUES ('Y', 5, 4, 6, 9, 1);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id, site_id) VALUES ('Y', 5, 4, 6, 10, 1);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id, site_id) VALUES ('Y', 5, 4, 6, 11, 1);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id, site_id) VALUES ('Y', 5, 4, 6, 12, 1);

-- user 5 will also have a few permissions for another department: should not show as a duplicate
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id, site_id) VALUES ('Y', 5, 1, 6, 1, 1);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id, site_id) VALUES ('Y', 5, 1, 6, 2, 1);

-- user 5 will also have a few permissions that are not active, in the other department; should not show at all
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id, site_id) VALUES ('N', 5, 1, 6, 3, 1);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id, site_id) VALUES ('N', 5, 1, 6, 4, 1);

COMMIT;

-- Dr. Robert Chase will have the Physician role, with access to clinical data mostly
--
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id, site_id) VALUES ('Y', 6, 6, 3, 1, 1);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id, site_id) VALUES ('Y', 6, 6, 3, 2, 1);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id, site_id) VALUES ('Y', 6, 6, 3, 6, 1);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id, site_id) VALUES ('Y', 6, 6, 3, 8, 1);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id, site_id) VALUES ('Y', 6, 6, 3, 9, 1);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id, site_id) VALUES ('Y', 6, 6, 3, 10, 1);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id, site_id) VALUES ('Y', 6, 6, 3, 11, 1);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id, site_id) VALUES ('Y', 6, 6, 3, 12, 1);

COMMIT;

-- Dr. Drake Ramoray will have the Physician role, with access to clinical data mostly
--
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id, site_id) VALUES ('Y', 7, 15, 3, 1, 1);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id, site_id) VALUES ('Y', 7, 15, 3, 2, 1);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id, site_id) VALUES ('Y', 7, 15, 3, 6, 1);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id, site_id) VALUES ('Y', 7, 15, 3, 8, 1);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id, site_id) VALUES ('Y', 7, 15, 3, 9, 1);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id, site_id) VALUES ('Y', 7, 15, 3, 10, 1);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id, site_id) VALUES ('Y', 7, 15, 3, 11, 1);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id, site_id) VALUES ('Y', 7, 15, 3, 12, 1);

COMMIT;

-- Peter Porter will have the Porter role, with very limited access to non-clinical data only
--
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id, site_id) VALUES ('Y', 8, 5, 1, 1, 1);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id, site_id) VALUES ('Y', 8, 5, 1, 3, 1);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id, site_id) VALUES ('Y', 8, 5, 1, 7, 1);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id, site_id) VALUES ('Y', 8, 5, 1, 12, 1);

COMMIT;

-- give Carol all the Intervention permissions
insert into user_permission (active_flag, users_id, department_id, role_id, permission_id, site_id) 
select 'Y', 2, 4, 3, id, 1 from permission where id > 100001;

-- give Lisa all the Intervention permissions
insert into user_permission (active_flag, users_id, department_id, role_id, permission_id, site_id) 
select 'Y', 5, 4, 3, id, 1 from permission where id > 100001;

-- Ian Valid will have... weird permissions for unit testing
--
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id, site_id) VALUES ('Y', 9, 15, 3, 1, 2);
INSERT INTO user_permission(active_flag, users_id, department_id, role_id, permission_id, site_id) VALUES ('Y', 9, 15, 3, 2, 2);

COMMIT;