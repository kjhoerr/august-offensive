<?php

declare(strict_types=1);

namespace AugustOffensive\model;

/**
 * Holds the credentials for connecting to the database
 */
class Cred
{
    /** @var string $host The destination host that holds the database. */
    private $host;
    /** @var string $port The destination port for the database on the host. */
    private $port;
    /** @var string $dbname The name of the database of which to connect. */
    private $dbname;
    /** @var string $login The username for accessing the database.
     *
     * It is recommended to change the login to a more restrictive account once
     * the tables have been created (e.g. an account that can only insert,
     * select, and update on that specific database).
     */
    private $login;
    /** @var string $password The password to the account for access.  */
    private $password;

    /**
     * Sets the values of the credentials.
     *
     * @return Cred
     */
    public function __construct ()
    {
        $host     = 'localhost';
        $port     = '5432';
        $dbname   = 'ao';
        $login    = 'r_access';
        $password = 'secret';
    }

    /**
     * Destructor ensures clean wipe of credentials from existing.
     *
     * @return void
     */
    public function __destruct ()
    {
        $host     = '';
        $port     = '';
        $dbname   = '';
        $login    = '';
        $password = '';
    }

    /**
     * Returns the value of the host of the database.
     *
     * @return string $host
     */
    public function getHost ()
    {
        return $host;
    }
    /**
     * Returns the value of the port of the host of which to connect.
     *
     * @return string $port
     */
    public function getPort ()
    {
        return $port;
    }
    /**
     * Returns the value of the name of the database of which to connect.
     *
     * @return string $dbname
     */
    public function getDBName () 
    {
        return $dbname;
    }
    /**
     * Returns the value of the username of the account of the database.
     *
     * @return string $login
     */
    public function getLogin () 
    {
        return $login;
    }
    /**
     * Returns the value of the password of the account of the database.
     *
     * @return string password
     */
    public function getPassword ()
    {
        return $password;
    }
}

// destroy as quickly as possible
$cred = new Cred();

