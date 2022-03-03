DROP PROCEDURE IF EXISTS `test_blake3_hash`;
DELIMITER $$
CREATE PROCEDURE `test_blake3_hash`()
BEGIN
    DECLARE i INT DEFAULT 0;
    DECLARE h CHAR(64) DEFAULT 'A';
    WHILE i < 10000 DO
        SET h = blake3_hash(h);
        SET i = i + 1;
    END WHILE;
    select h;
END$$
DELIMITER ;

DROP PROCEDURE IF EXISTS `test_sha2_256_hash`;
DELIMITER $$
CREATE PROCEDURE `test_sha2_256_hash`()
BEGIN
    DECLARE i INT DEFAULT 0;
    DECLARE h CHAR(64) DEFAULT 'A';
    WHILE i < 10000 DO
        SET h = sha2(h, 256);
        SET i = i + 1;
    END WHILE;
    select h;
END$$
DELIMITER ;


CALL test_blake3_hash();
CALL test_sha2_256_hash();
