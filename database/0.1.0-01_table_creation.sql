USE `magnesie_image_storage`;

CREATE TABLE IF NOT EXISTS `users` (
    `id` INT PRIMARY KEY NOT NULL AUTO_INCREMENT,
    `name` VARCHAR(100) NOT NULL
);

CREATE TABLE IF NOT EXISTS `sites` (
    `id` INT PRIMARY KEY NOT NULL AUTO_INCREMENT,
    `name` VARCHAR(100) NOT NULL,
    `location` POINT NOT NULL,
    `details` VARCHAR(200) NOT NULL
);

CREATE TABLE IF NOT EXISTS `areas` (
    `id` INT PRIMARY KEY NOT NULL AUTO_INCREMENT,
    `name` VARCHAR(100) NOT NULL,
    `site_id` INT NOT NULL,
    FOREIGN KEY (`site_id`) REFERENCES `sites`(`id`) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS `submissions` (
    `id` INT PRIMARY KEY NOT NULL AUTO_INCREMENT,
    `user_id` INT NOT NULL,
    `area_id` INT NOT NULL,
    `submission_date` TIMESTAMP NOT NULL,
    `status` VARCHAR(10) NOT NULL,
    FOREIGN KEY (`user_id`) REFERENCES `users`(`id`) ON DELETE CASCADE,
    FOREIGN KEY (`area_id`) REFERENCES `areas`(`id`) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS `photos` (
    `id` INT PRIMARY KEY NOT NULL AUTO_INCREMENT,
    `file_name` VARCHAR(100) NOT NULL,
    `submission_id` INT NOT NULL,
    `path` VARCHAR(100) NOT NULL,
    FOREIGN KEY (`submission_id`) REFERENCES `submissions`(`id`) ON DELETE CASCADE
);

COMMIT;