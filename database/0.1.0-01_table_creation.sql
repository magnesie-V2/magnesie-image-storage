USE magnesie_image_storage;

CREATE TABLE IF NOT EXISTS submissions (
    id INT PRIMARY KEY NOT NULL AUTO_INCREMENT,
    name VARCHAR(50) NOT NULL,
    submission_date TIMESTAMP NOT NULL,
    status VARCHAR(10) NOT NULL
);

CREATE TABLE IF NOT EXISTS photos (
    id INT PRIMARY KEY NOT NULL AUTO_INCREMENT,
    file_name VARCHAR(100) NOT NULL,
    submission_id INT NOT NULL,
    path VARCHAR(100) NOT NULL,
    FOREIGN KEY (submission_id) REFERENCES submissions(id) ON DELETE CASCADE
);

COMMIT;

USE magnesie_result_storage;

CREATE TABLE IF NOT EXISTS results (
    id INT PRIMARY KEY NOT NULL,
    name VARCHAR(50) NOT NULL,
    model_path VARCHAR(200) NOT NULL,
    texture_path VARCHAR(200) NOT NULL,
    total_consumption_joules INT NOT NULL
);

COMMIT;
