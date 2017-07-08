<?php

declare(strict_types=1);

namespace AugustOffensive\Model;

/**
 * Model connection class for connecting to database via PDO.
 */
class Connection
{
    /** @var \PDO $_conn PDO connection to database. */
    private $conn;
    /**
     * Initiates connection to PostGreSQL database.
     *
     * @return Connection
     */
    public function __construct ()
    {
        // Establish connection to db
        include 'creds.php';

        try {
	        $conn = new \PDO(
                "pgsql: host=" . $cred->host .
                    (($cred->port !== '') ? ";port=" . $cred->port : '') .
                    ";dbname=" . $cred->dbName,
                $cred->login,
                $cred->password
            );
            // we destroy $cred as quickly as possible
            $cred = null;
        } catch (\PDOException $err) {
            // we destroy $cred as quickly as possible
            $cred = null;
            die(json_encode(array("Result-Type" => "ERROR", "Content" => array($err->getMessage()))));
        }
        return $this;
    }
}
