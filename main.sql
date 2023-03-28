/*
 Navicat Premium Data Transfer

 Source Server         : fast_rust
 Source Server Type    : SQLite
 Source Server Version : 3030001
 Source Schema         : main

 Target Server Type    : SQLite
 Target Server Version : 3030001
 File Encoding         : 65001

 Date: 28/03/2023 16:53:20
*/

PRAGMA foreign_keys = false;

-- ----------------------------
-- Table structure for _menu_old_20230327
-- ----------------------------
DROP TABLE IF EXISTS "_menu_old_20230327";
CREATE TABLE "_menu_old_20230327" (
  "menu_id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  "menu_name" TEXT,
  "parent_id" INTEGER,
  "path" TEXT,
  "icon" TEXT,
  "remark" TEXT,
  "status" integer,
  "create_time" TEXT,
  "index_no" INTEGER,
  "user_id" INTEGER
);

-- ----------------------------
-- Records of _menu_old_20230327
-- ----------------------------
INSERT INTO "_menu_old_20230327" VALUES (1, '系统管理', 0, '/system', 'sdf', '测试', 1, '2023-03-27 00:00:00', 1, 1);
INSERT INTO "_menu_old_20230327" VALUES (2, '用户管理', 0, '/user', 'sdf', '测试', 1, '2023-03-27 00:00:00', 1, 1);
INSERT INTO "_menu_old_20230327" VALUES (3, '菜单管理', 0, '/menu', 'sdf', '测试', 1, '2023-03-27 00:00:00', 1, 1);
INSERT INTO "_menu_old_20230327" VALUES (4, '单位管理', 0, '/dept', 'sdf', '测试', 1, '2023-03-27 00:00:00', 1, 1);
INSERT INTO "_menu_old_20230327" VALUES (5, '第一单位管理', 4, '/dept', 'sdf', '测试', 1, '2023-03-27 00:00:00', 1, 1);
INSERT INTO "_menu_old_20230327" VALUES (6, '第二单位管理', 4, '/dept', 'sdf', '测试', 1, '2023-03-27 00:00:00', 1, 1);
INSERT INTO "_menu_old_20230327" VALUES (7, '第三单位管理', 4, '/dept', 'sdf', '测试', 1, '2023-03-27 00:00:00', 1, 1);

-- ----------------------------
-- Table structure for menu
-- ----------------------------
DROP TABLE IF EXISTS "menu";
CREATE TABLE "menu" (
  "menu_id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  "menu_name" TEXT,
  "parent_id" INTEGER,
  "path" TEXT,
  "icon" TEXT,
  "remark" TEXT,
  "status" integer,
  "create_time" TEXT,
  "index_no" INTEGER,
  "user_id" INTEGER,
  "clazz" TEXT
);

-- ----------------------------
-- Records of menu
-- ----------------------------
INSERT INTO "menu" VALUES (1, '系统管理', 0, '/system', 'sdf', '测试', 1, '2023-03-27 00:00:00', 1, 1, NULL);
INSERT INTO "menu" VALUES (2, '用户管理', 0, '/user', 'sdf', '测试', 1, '2023-03-27 00:00:00', 1, 1, NULL);
INSERT INTO "menu" VALUES (3, '菜单管理', 0, '/menu', 'sdf', '测试', 1, '2023-03-27 00:00:00', 1, 1, NULL);
INSERT INTO "menu" VALUES (4, '单位管理', 0, '/dept', 'sdf', '测试', 1, '2023-03-27 00:00:00', 1, 1, NULL);
INSERT INTO "menu" VALUES (5, '第一单位管理', 4, '/dept', 'sdf', '测试', 1, '2023-03-27 00:00:00', 1, 1, NULL);
INSERT INTO "menu" VALUES (6, '第二单位管理', 4, '/dept', 'sdf', '测试', 1, '2023-03-27 00:00:00', 1, 1, NULL);
INSERT INTO "menu" VALUES (7, '第三单位管理', 4, '/dept', 'sdf', '测试', 1, '2023-03-27 00:00:00', 1, 1, NULL);
INSERT INTO "menu" VALUES (8, '第三单位管理', 7, '/dept', 'sdf', '测试', 1, '2023-03-27 00:00:00', 1, 1, NULL);

-- ----------------------------
-- Table structure for sqlite_sequence
-- ----------------------------
DROP TABLE IF EXISTS "sqlite_sequence";
CREATE TABLE "sqlite_sequence" (
  "name",
  "seq"
);

-- ----------------------------
-- Records of sqlite_sequence
-- ----------------------------
INSERT INTO "sqlite_sequence" VALUES ('_menu_old_20230327', 7);
INSERT INTO "sqlite_sequence" VALUES ('menu', 8);

-- ----------------------------
-- Table structure for student
-- ----------------------------
DROP TABLE IF EXISTS "student";
CREATE TABLE "student" (
  "student_id" integer,
  "name" text,
  "class" text,
  "mobile" text,
  "address" text
);

-- ----------------------------
-- Records of student
-- ----------------------------

-- ----------------------------
-- Table structure for user
-- ----------------------------
DROP TABLE IF EXISTS "user";
CREATE TABLE "user" (
  "user_id" integer,
  "user_name" text,
  "password" text,
  "create_time" text,
  "status" integer,
  CONSTRAINT "user_pk" PRIMARY KEY ("user_id")
);

-- ----------------------------
-- Records of user
-- ----------------------------
INSERT INTO "user" VALUES (2, '我是你大爷', '123456', '2023-03-26 17:45:15', 1);
INSERT INTO "user" VALUES (3, 'lu', '123456', '2023-03-26 17:45:22', 1);
INSERT INTO "user" VALUES (4, 'lan', '123456', '2023-03-26 17:45:26', 1);
INSERT INTO "user" VALUES (5, 'zi', '123456', '2023-03-26 17:45:32', 1);
INSERT INTO "user" VALUES (6, 'zi', '123456', '2023-03-26 18:57:55', 1);
INSERT INTO "user" VALUES (7, '你是谁', '123456', '2023-03-26 21:13:42', 1);

-- ----------------------------
-- Auto increment value for _menu_old_20230327
-- ----------------------------
UPDATE "sqlite_sequence" SET seq = 7 WHERE name = '_menu_old_20230327';

-- ----------------------------
-- Auto increment value for menu
-- ----------------------------
UPDATE "sqlite_sequence" SET seq = 8 WHERE name = 'menu';

PRAGMA foreign_keys = true;
