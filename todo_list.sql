-- MySQL dump 10.13  Distrib 8.0.38, for Win64 (x86_64)
--
-- Host: 127.0.0.1    Database: todo_list
-- ------------------------------------------------------
-- Server version	9.0.1

/*!40101 SET @OLD_CHARACTER_SET_CLIENT=@@CHARACTER_SET_CLIENT */;
/*!40101 SET @OLD_CHARACTER_SET_RESULTS=@@CHARACTER_SET_RESULTS */;
/*!40101 SET @OLD_COLLATION_CONNECTION=@@COLLATION_CONNECTION */;
/*!50503 SET NAMES utf8 */;
/*!40103 SET @OLD_TIME_ZONE=@@TIME_ZONE */;
/*!40103 SET TIME_ZONE='+00:00' */;
/*!40014 SET @OLD_UNIQUE_CHECKS=@@UNIQUE_CHECKS, UNIQUE_CHECKS=0 */;
/*!40014 SET @OLD_FOREIGN_KEY_CHECKS=@@FOREIGN_KEY_CHECKS, FOREIGN_KEY_CHECKS=0 */;
/*!40101 SET @OLD_SQL_MODE=@@SQL_MODE, SQL_MODE='NO_AUTO_VALUE_ON_ZERO' */;
/*!40111 SET @OLD_SQL_NOTES=@@SQL_NOTES, SQL_NOTES=0 */;

--
-- Table structure for table `log_table`
--

DROP TABLE IF EXISTS `log_table`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `log_table` (
  `log_id` int NOT NULL AUTO_INCREMENT,
  `mission_id` varchar(45) NOT NULL,
  `log_messege` varchar(45) NOT NULL,
  `log_date` varchar(45) NOT NULL,
  PRIMARY KEY (`log_id`),
  UNIQUE KEY `log_id_UNIQUE` (`log_id`)
) ENGINE=InnoDB AUTO_INCREMENT=4 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `log_table`
--

LOCK TABLES `log_table` WRITE;
/*!40000 ALTER TABLE `log_table` DISABLE KEYS */;
INSERT INTO `log_table` VALUES (1,'1','第一次測試','2024/10/14'),(2,'1','第一次測試','2024/10/14'),(3,'1','第一次salvo','2024/10/14');
/*!40000 ALTER TABLE `log_table` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `mission_table`
--

DROP TABLE IF EXISTS `mission_table`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `mission_table` (
  `id` int NOT NULL AUTO_INCREMENT,
  `mission_name` varchar(45) NOT NULL,
  `worker_name` varchar(45) NOT NULL,
  `state` varchar(45) NOT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `id_UNIQUE` (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=7 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `mission_table`
--

LOCK TABLES `mission_table` WRITE;
/*!40000 ALTER TABLE `mission_table` DISABLE KEYS */;
INSERT INTO `mission_table` VALUES (1,'test_rust','Daniel','nonstart'),(2,'測試rust','Daniel','nonstart'),(5,'測試rust','Daniel','nonstart'),(6,'測試rust1111','Daniel','nonstart');
/*!40000 ALTER TABLE `mission_table` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `name_table`
--

DROP TABLE IF EXISTS `name_table`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `name_table` (
  `name` char(30) NOT NULL,
  PRIMARY KEY (`name`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `name_table`
--

LOCK TABLES `name_table` WRITE;
/*!40000 ALTER TABLE `name_table` DISABLE KEYS */;
INSERT INTO `name_table` VALUES ('Daniel'),('daniell'),('godkiller'),('godkiller1'),('godkiller2'),('Sandy');
/*!40000 ALTER TABLE `name_table` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `project_table`
--

DROP TABLE IF EXISTS `project_table`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `project_table` (
  `案號` varchar(30) NOT NULL,
  `案名` varchar(45) NOT NULL,
  `說明` varchar(200) DEFAULT 'None',
  `Delivery pecentage` varchar(45) DEFAULT 'None',
  `客戶` varchar(45) DEFAULT 'None',
  `通路` varchar(45) DEFAULT 'None',
  `開案日期` varchar(45) DEFAULT 'None',
  `需求回覆日期` varchar(45) DEFAULT 'None',
  `負責PM` varchar(45) DEFAULT 'None',
  `開案人員` varchar(45) DEFAULT 'None',
  `Agent` varchar(45) DEFAULT 'None',
  `Kernel` varchar(45) DEFAULT 'None',
  `Algorithm` varchar(45) DEFAULT 'None',
  `software` varchar(45) DEFAULT 'None',
  `module` varchar(45) DEFAULT 'None',
  `loading` varchar(45) DEFAULT 'None',
  `Project_status` varchar(45) DEFAULT 'None',
  `Project note` varchar(1000) DEFAULT 'None',
  `hardware-computer` varchar(45) DEFAULT 'None',
  `hardware-TX type` varchar(45) DEFAULT 'None',
  `hardware-RX type` varchar(45) DEFAULT 'None',
  `SI partner` varchar(45) DEFAULT 'None',
  `Phase0-開案開始` varchar(45) DEFAULT 'None',
  `Phase0-確認規格` varchar(45) DEFAULT 'None',
  `RFP time(day)` varchar(45) DEFAULT 'None',
  `Phase1-需求評估開始` varchar(45) DEFAULT 'None',
  `Phase1-評估結束時間` varchar(45) DEFAULT 'None',
  `PoC time(day)` varchar(45) DEFAULT 'None',
  `Phase2-方案評估開始` varchar(45) DEFAULT 'None',
  `Phase2-方案評估結束` varchar(45) DEFAULT 'None',
  `SOW time(day)` varchar(45) DEFAULT 'None',
  `Phase3-商務報價評估(Quotation)` varchar(45) DEFAULT 'None',
  `Phase3-商務報價提供(Quotation)` varchar(45) DEFAULT 'None',
  `Business time(day)` varchar(45) DEFAULT 'None',
  `Phase4-工程開始(Get PO)` varchar(45) DEFAULT 'None',
  `Phase4-工程結束(Acceptance)` varchar(45) DEFAULT 'None',
  `Engineering time(day)` varchar(45) DEFAULT 'None',
  `Phase0 document (RFQ simple)` varchar(45) DEFAULT 'None',
  `Phase1 document (PoC report)` varchar(45) DEFAULT 'None',
  `Phase2 document (SoW/RFQ complete)` varchar(45) DEFAULT 'None',
  `Phase3 document (Quotation)` varchar(45) DEFAULT 'None',
  `Phase4 document (PO)` varchar(45) DEFAULT 'None',
  `Total amount(未稅)` varchar(45) DEFAULT 'None',
  `Deposit status` varchar(45) DEFAULT 'None',
  `Deposit pecentage` varchar(45) DEFAULT 'None',
  `Deposit amount` varchar(45) DEFAULT 'None',
  `time1` varchar(45) DEFAULT 'None',
  `Delivery status` varchar(45) DEFAULT 'None',
  `Delivery amount` varchar(45) DEFAULT 'None',
  `time2` varchar(45) DEFAULT 'None',
  `Acceptance status` varchar(45) DEFAULT 'None',
  `Acceptance pecentage` varchar(45) DEFAULT 'None',
  `Acceptance amount` varchar(45) DEFAULT 'None',
  `time3` varchar(45) DEFAULT 'None',
  `交付類型` varchar(45) DEFAULT 'None',
  PRIMARY KEY (`案號`),
  UNIQUE KEY `案號_UNIQUE` (`案號`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `project_table`
--

LOCK TABLES `project_table` WRITE;
/*!40000 ALTER TABLE `project_table` DISABLE KEYS */;
INSERT INTO `project_table` VALUES ('123','proj1','None','None','None','None','None','None','None','None','None','None','None','None','None','None','None','None','None','None','None','None','None','None','None','None','None','None','None','None','None','None','None','None','None','None','None','None','None','None','None','None','None','None','None','None','None','None','None','None','None','None','None','None','None');
/*!40000 ALTER TABLE `project_table` ENABLE KEYS */;
UNLOCK TABLES;
/*!40103 SET TIME_ZONE=@OLD_TIME_ZONE */;

/*!40101 SET SQL_MODE=@OLD_SQL_MODE */;
/*!40014 SET FOREIGN_KEY_CHECKS=@OLD_FOREIGN_KEY_CHECKS */;
/*!40014 SET UNIQUE_CHECKS=@OLD_UNIQUE_CHECKS */;
/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40101 SET CHARACTER_SET_RESULTS=@OLD_CHARACTER_SET_RESULTS */;
/*!40101 SET COLLATION_CONNECTION=@OLD_COLLATION_CONNECTION */;
/*!40111 SET SQL_NOTES=@OLD_SQL_NOTES */;

-- Dump completed on 2024-10-19 18:29:04
