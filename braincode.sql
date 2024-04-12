-- phpMyAdmin SQL Dump
-- version 5.2.1
-- https://www.phpmyadmin.net/
--
-- Host: 127.0.0.1
-- Generation Time: Apr 12, 2024 at 05:32 AM
-- Server version: 8.0.36-0ubuntu0.22.04.1
-- PHP Version: 8.2.4

SET SQL_MODE = "NO_AUTO_VALUE_ON_ZERO";
START TRANSACTION;
SET time_zone = "+00:00";


/*!40101 SET @OLD_CHARACTER_SET_CLIENT=@@CHARACTER_SET_CLIENT */;
/*!40101 SET @OLD_CHARACTER_SET_RESULTS=@@CHARACTER_SET_RESULTS */;
/*!40101 SET @OLD_COLLATION_CONNECTION=@@COLLATION_CONNECTION */;
/*!40101 SET NAMES utf8mb4 */;

--
-- Database: `braincode`
--

-- --------------------------------------------------------

--
-- Table structure for table `contact`
--

CREATE TABLE `contact` (
  `id` varchar(36) NOT NULL,
  `location` varchar(255) NOT NULL,
  `phone_number` varchar(255) NOT NULL,
  `email` varchar(255) NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

--
-- Dumping data for table `contact`
--

INSERT INTO `contact` (`id`, `location`, `phone_number`, `email`) VALUES
('eca8bd0b-f86e-11ee-8776-409f38c75905', 'Banjarkerta, Karanganyar', '082220484108', 'singgihbudi.sbh@gmail.com');

-- --------------------------------------------------------

--
-- Table structure for table `edukasi`
--

CREATE TABLE `edukasi` (
  `id` varchar(36) NOT NULL,
  `title` varchar(255) NOT NULL,
  `major` varchar(255) NOT NULL,
  `periode` varchar(255) NOT NULL,
  `description` longtext NOT NULL,
  `logo` varchar(255) NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

--
-- Dumping data for table `edukasi`
--

INSERT INTO `edukasi` (`id`, `title`, `major`, `periode`, `description`, `logo`) VALUES
('7a738f9c-f86e-11ee-8776-409f38c75905', 'Institut Teknologi Telkom Purwokerto\r\n', 'Software Engineering\r\n\r\n', 'Sep 2020 - Feb 2024, GPA 3.94\r\n\r\n', 'Learn to be a software engineer such as UML : use case diagrams, class diagrams, activity diagrams, sequence diagrams. ERD and DFD. Learn the programming language such as java, javascript, php. Learn markup languages html, css, xml. Learn framework laravel, reactjs, nextjs, nodejs, tailwind css, bootstrap. Learn application testing such as black box testing and white box testing. Learn databases such as data definition language and data manipulation language\r\n\r\n', 'https://www.profile.sibudi.my.id/assets/1110-JEGJWPpC.jpg');

-- --------------------------------------------------------

--
-- Table structure for table `experience`
--

CREATE TABLE `experience` (
  `id` varchar(36) NOT NULL,
  `periode` varchar(255) NOT NULL,
  `title` varchar(255) NOT NULL,
  `division` varchar(255) NOT NULL,
  `description` longtext NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

--
-- Dumping data for table `experience`
--

INSERT INTO `experience` (`id`, `periode`, `title`, `division`, `description`) VALUES
('556bb75e-c8bf-4ef8-aab1-06c3f99b72ca', 'Sep 2020 - Feb 2024', 'Software Engineering Student Association', 'Media and information division staff', 'As a member of the Media and Information Division staff, responsibilities include designing Instagram feed layouts, creating content, developing posters, engaging in photography, and contributing to project management tasks using tools like Figma and Trello for efficient organization.'),
('905f3541-3e5e-4df5-8771-36c9ff9ff326', 'August 2022 - December 2022', 'Diesnatalis 6th Software Engineering', 'PDD or publication-decoration-documentation division coordinator', 'As the PDD Division Coordinator, responsibilities include coordinating staff activities, overseeing project management tasks for publication, decoration, and documentation, creating posters, engaging in photography, designing Instagram feed layouts, generating content, and disseminating information about managed events.'),
('c3658f85-1e20-4eda-804b-086e042d3fd0', 'August 2022 - December 2022', 'Ruangguru Indonesia - Study Independent', 'FrontEnd Engineering', 'Learn HTML, CSS, Tailwind CSS, JavaScript, ReactJS, and Node.js for comprehensive web development, applying skills in a final project, gaining practical experience through a certified apprenticeship, and staying current with industry trends through independent study and community engagement.'),
('cad26f05-719d-4691-b0b9-46a185760cfc', 'August 2021 - December 2021', 'Diesnatalis 5th Software Engineering', 'PDD or publication-decoration-documentation staff', 'As a member of the PDD (Publication-Decoration-Documentation) staff, responsibilities encompass designing Instagram feed layouts, creating content for Diesnatalis events, utilizing design tools like Figma and Canva for visually appealing designs, and serving as a photographer to document memorable moments during events.'),
('e30b7ee5-498d-472a-a55d-bd6026f24b96', 'February 2022 - June 2022', 'Software Engineering Student Association', 'Education and development division staff', 'As a member of The Education and Development Division staff, responsibilities include enhancing students\' software engineering hardskills, conducting training sessions, providing guidance for Gemastik competitions, and assisting with exam preparations.');

-- --------------------------------------------------------

--
-- Table structure for table `hero`
--

CREATE TABLE `hero` (
  `id` varchar(36) NOT NULL,
  `title` varchar(255) NOT NULL,
  `description` longtext NOT NULL,
  `image` varchar(255) NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

--
-- Dumping data for table `hero`
--

INSERT INTO `hero` (`id`, `title`, `description`, `image`) VALUES
('e2f29faf-654b-4be5-b680-d3acfb0c0649', 'Junior\r\nWebsite\r\nDeveloper', 'Fresh Graduated Software Engineer From Institut Teknologi Telkom Purwokerto\r\n\r\n', 'https://www.profile.sibudi.my.id/assets/3x4-0cXuwhix.jpeg');

-- --------------------------------------------------------

--
-- Table structure for table `portofolio`
--

CREATE TABLE `portofolio` (
  `id` varchar(36) NOT NULL,
  `title` varchar(255) NOT NULL,
  `description` longtext NOT NULL,
  `skills` text NOT NULL,
  `link` longtext NOT NULL,
  `image` varchar(255) NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

--
-- Dumping data for table `portofolio`
--

INSERT INTO `portofolio` (`id`, `title`, `description`, `skills`, `link`, `image`) VALUES
('1', 'Website Event, August 2023 - January 2024\r\n', 'Building a website for connecting college students with sponsorships as the final assignment project, incorporating MySQL, Laravel, Tailwind CSS, and Javascript\r\n', 'Laravel, JavaScript, Fullstack, Monolith, Tailwind CSS, MySQL\r\n', 'https://github.com/Mr-buddyy/Skripsi_Rev21', 'https://www.profile.sibudi.my.id/assets/website-event-SVfoY2Lo.png'),
('2', 'Website Stoik, December 2023 - January 2024\r\n', 'As a front-end developer, I specialize in crafting visually appealing website interfaces using the Next.js and Tailwind CSS frameworks. I seamlessly integrate backend APIs, incorporating essential functionalities like search and sorting to enhance user experience.\r\n', 'Laravel, NextJs, JavaScript, Fullstack, Tailwind CSS, MySQL\r\n', 'https://github.com/Mr-buddyy/Tubes-Front-End', 'https://www.profile.sibudi.my.id/assets/website-stoik-w7M7L2Ls.png'),
('3', 'Website Climate Aware, December 2023 - January 2024\r\n', 'As Backend developer. Provisioning endpoints based on requirements while ensuring security, implementing business logic, handling errors, conducting backend testing, and managing databases. In the context of utilizing Laravel technology, MySQL is employed for database management. Also designing the logo\r\n', 'Laravel, NextJs, JavaScript, Fullstack, MySQL\r\n', 'https://github.com/mirfani340/climate-aware.id', 'https://www.profile.sibudi.my.id/assets/website-climateaware-84qGBlMj.png'),
('4', 'Website Gamis, November 2023- December 2023\r\n', 'As Backend developer. Provisioning endpoints based on requirements while ensuring security, implementing business logic, handling errors, conducting backend testing, and managing databases. In the context of utilizing Laravel technology, MySQL is employed for database management. Also designing the logo\r\n', 'Laravel, NextJs, JavaScript, Fullstack, Tailwind CSS, MySQL\r\n', 'https://github.com/Mr-buddyy/Gamis', 'https://www.profile.sibudi.my.id/assets/website-gamis-eWnE8Nhj.png'),
('5', 'Website Desa, May 2022 - July 2022\r\n', 'Developing a website for Central Sokaraja village as a project for the web design and programming midterm test, utilizing MySQL, PHP, HTML, CSS, and Tailwind\r\n', 'PHP, JavaScript, CSS, Fullstack, MySQL\r\n', 'https://github.com/raynaldizf/website-desa-sokaraja', 'https://www.profile.sibudi.my.id/assets/website-desa-dmo8B4F7.png');

-- --------------------------------------------------------

--
-- Table structure for table `skill`
--

CREATE TABLE `skill` (
  `id` varchar(36) NOT NULL,
  `skills` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

--
-- Dumping data for table `skill`
--

INSERT INTO `skill` (`id`, `skills`) VALUES
('b0342d3b-f877-11ee-8776-409f38c75905', 'UI Design'),
('b03435ec-f877-11ee-8776-409f38c75905', 'Web Development'),
('b0343dd1-f877-11ee-8776-409f38c75905', 'Responsive web design'),
('b03445ba-f877-11ee-8776-409f38c75905', 'HTML'),
('b0344cdc-f877-11ee-8776-409f38c75905', 'JavaScript'),
('b034538c-f877-11ee-8776-409f38c75905', 'Laravel'),
('b0345aa0-f877-11ee-8776-409f38c75905', 'Github'),
('b0345d69-f877-11ee-8776-409f38c75905', 'ReactJS'),
('b0346222-f877-11ee-8776-409f38c75905', 'Tailwind CSS'),
('b03464db-f877-11ee-8776-409f38c75905', 'MySQL'),
('b034699c-f877-11ee-8776-409f38c75905', 'NextJS'),
('b0346c4d-f877-11ee-8776-409f38c75905', 'CSS');

-- --------------------------------------------------------

--
-- Table structure for table `__diesel_schema_migrations`
--

CREATE TABLE `__diesel_schema_migrations` (
  `version` varchar(50) NOT NULL,
  `run_on` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

--
-- Dumping data for table `__diesel_schema_migrations`
--

INSERT INTO `__diesel_schema_migrations` (`version`, `run_on`) VALUES
('20240409045845', '2024-04-11 16:08:07');

--
-- Indexes for dumped tables
--

--
-- Indexes for table `contact`
--
ALTER TABLE `contact`
  ADD PRIMARY KEY (`id`);

--
-- Indexes for table `edukasi`
--
ALTER TABLE `edukasi`
  ADD PRIMARY KEY (`id`);

--
-- Indexes for table `experience`
--
ALTER TABLE `experience`
  ADD PRIMARY KEY (`id`);

--
-- Indexes for table `hero`
--
ALTER TABLE `hero`
  ADD PRIMARY KEY (`id`);

--
-- Indexes for table `portofolio`
--
ALTER TABLE `portofolio`
  ADD PRIMARY KEY (`id`);

--
-- Indexes for table `skill`
--
ALTER TABLE `skill`
  ADD PRIMARY KEY (`id`);

--
-- Indexes for table `__diesel_schema_migrations`
--
ALTER TABLE `__diesel_schema_migrations`
  ADD PRIMARY KEY (`version`);
COMMIT;

/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40101 SET CHARACTER_SET_RESULTS=@OLD_CHARACTER_SET_RESULTS */;
/*!40101 SET COLLATION_CONNECTION=@OLD_COLLATION_CONNECTION */;
