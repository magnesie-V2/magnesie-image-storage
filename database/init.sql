CREATE SCHEMA IF NOT EXISTS `magnesie_image_storage`;

CREATE TABLE `magnesie_image_storage`.`user` (
    `id` INT PRIMARY KEY NOT NULL AUTO_INCREMENT,
    `name` VARCHAR(100)
);

CREATE TABLE `magnesie_image_storage`.`site` (
    `id` INT PRIMARY KEY NOT NULL AUTO_INCREMENT,
    `name` VARCHAR(100),
    `location` POINT,
    `details` VARCHAR(200)
);

CREATE TABLE `magnesie_image_storage`.`area` (
    `id` INT PRIMARY KEY NOT NULL AUTO_INCREMENT,
    `name` VARCHAR(100),
    `siteId` INT,
    FOREIGN KEY (`siteId`) REFERENCES `site`(`id`) ON DELETE CASCADE
);

CREATE TABLE `magnesie_image_storage`.`submission` (
    `id` INT PRIMARY KEY NOT NULL AUTO_INCREMENT,
    `userId` INT,
    `areaId` INT,
    `submissionDate` DATETIME,
    `status` VARCHAR(10),
    FOREIGN KEY (`userId`) REFERENCES `user`(`id`) ON DELETE CASCADE,
    FOREIGN KEY (`areaId`) REFERENCES `area`(`id`) ON DELETE CASCADE
);

CREATE TABLE `magnesie_image_storage`.`photo` (
    `id` INT PRIMARY KEY NOT NULL AUTO_INCREMENT,
    `fileName` VARCHAR(100),
    `submissionId` INT,
    `path` VARCHAR(100),
    FOREIGN KEY (`submissionId`) REFERENCES `submission`(`id`) ON DELETE CASCADE
);
