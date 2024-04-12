CREATE TABLE edukasi (
    id VARCHAR(36) PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    major VARCHAR(255) NOT NULL,
    periode VARCHAR(255) NOT NULL,
    description LONGTEXT NOT NULL,
    logo VARCHAR(255) NOT NULL
);

CREATE TABLE experience (
    id VARCHAR(36) PRIMARY KEY,
    periode VARCHAR(255) NOT NULL,
    title VARCHAR(255) NOT NULL,
    division VARCHAR(255) NOT NULL,
    description LONGTEXT NOT NULL
);

CREATE TABLE contact (
    id VARCHAR(36) PRIMARY KEY,
    location VARCHAR(255) NOT NULL,
    phone_number VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL
);

CREATE TABLE portofolio (
    id VARCHAR(36) PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    description LONGTEXT NOT NULL,
    skills TEXT NOT NULL,
    link LONGTEXT NOT NULL,
    image VARCHAR(255) NOT NULL
);

CREATE TABLE hero (
    id VARCHAR(36) PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    description LONGTEXT NOT NULL,
    image VARCHAR(255) NOT NULL
);

CREATE TABLE skill (
    id VARCHAR(36) PRIMARY KEY,
    skills VARCHAR(255) NOT NULL,
);