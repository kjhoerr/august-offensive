<?php

declare(strict_types=1);

namespace AugustOffensive\model;

/**
 * Model connection class for connecting to database via PDO.
 */
class Connection
{
    /** @var PDO $_conn \PDO connection to database. */
    private $_conn;
    /**
     * Initiates connection to PostGreSQL database.
     *
     * @return Connection
     */
    public function __construct ()
    {
        // Establish connection to db
        include './creds.php';

        try {
	        $_conn = new \PDO(
                "pgsql: host=" . $cred->getHost() .
                    (($cred->getPort() !== '') ? ";port=" . $cred->getPort() : '') .
                    ";dbname=" . $cred->getDBName(),
                $cred->getLogin(),
                $cred->getPassword()
            );
            // we destroy $cred as quickly as possible
            $cred = null;
        } catch (\PDOException $e) {
            // we destroy $cred as quickly as possible
            $cred = null;
            die(json_encode(array("Result-Type" => "Error", "Content" => array($e))));
        }
    }
}
