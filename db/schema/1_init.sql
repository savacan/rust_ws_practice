CREATE TABLE test_table
(
    `id`                 BIGINT       NOT NULL AUTO_INCREMENT PRIMARY KEY,
    `name`               VARCHAR(255) NOT NULL,
    `created_at`         DATETIME     NOT NULL default CURRENT_TIMESTAMP,
    `updated_at`         DATETIME     NOT NULL default CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4;

CREATE TABLE lessors
(
    `id`                 BIGINT       NOT NULL AUTO_INCREMENT PRIMARY KEY,
    `name`               VARCHAR(255) NOT NULL,
    `created_at`         DATETIME     NOT NULL default CURRENT_TIMESTAMP,
    `updated_at`         DATETIME     NOT NULL default CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4;

CREATE TABLE users
(
    `id`           BIGINT       NOT NULL AUTO_INCREMENT PRIMARY KEY,
    `name`         VARCHAR(255) NOT NULL,
    `lessor_id`    BIGINT       NOT NULL,
    `created_at`   DATETIME     NOT NULL default CURRENT_TIMESTAMP,
    `updated_at`   DATETIME     NOT NULL default CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    KEY `fk_users_lessors` (`lessor_id`),
    CONSTRAINT `fk_users_lessors` FOREIGN KEY (`lessor_id`) REFERENCES `lessors` (`id`) ON DELETE RESTRICT ON UPDATE CASCADE
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4;

CREATE TABLE `buildings`
(
    `id`                      BIGINT       NOT NULL AUTO_INCREMENT PRIMARY KEY,
    `lessor_id`               BIGINT       NOT NULL,
    `name`                    VARCHAR(255) NOT NULL,

    # 所在地
    `prefecture` VARCHAR(255) NOT NULL, # 都道府県
    `ward`       VARCHAR(255) NOT NULL, # 市区町村
    `city`       VARCHAR(255) NOT NULL, # 町名
    `block`      VARCHAR(255) DEFAULT NULL, # 以降の所在地

    `created_at`              DATETIME     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    `updated_at`              DATETIME     NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    KEY `fk_buildings_lessors` (`lessor_id`),
    CONSTRAINT `fk_buildings_lessors` FOREIGN KEY (`lessor_id`) REFERENCES `lessors` (`id`) ON DELETE RESTRICT ON UPDATE CASCADE
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4;

CREATE TABLE `askings`
(

    `id`                              BIGINT         NOT NULL AUTO_INCREMENT PRIMARY KEY,
    `building_id`                     BIGINT         NOT NULL,
    `area`                      DECIMAL(10, 2) NOT NULL,
    `rent`                      INT(11)        DEFAULT NULL,
    `created_at`                      DATETIME       NOT NULL DEFAULT CURRENT_TIMESTAMP,
    `updated_at`                      DATETIME       NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    KEY `fk_askings_buildings` (`building_id`),
    CONSTRAINT `fk_askings_buildings` FOREIGN KEY (`building_id`) REFERENCES `buildings` (`id`) ON DELETE RESTRICT ON UPDATE CASCADE
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4;