<?php

declare(strict_types=1);

namespace AugustOffensive;

include 'private/model/Connection.php';
include 'private/view/Result.php';
include 'private/controller/Controller.php';

use AugustOffensive\view;
use AugustOffensive\model;

/**
 * Constructive controller and initializer API for the service.
 */
class Api 
{
    /** @var model\Connection $connection the model database interface */
    private $connection;

    /** @var view\Result $view the view interface that outputs result of the query */
    private $view;

    /**
     * Initiates database connection and forwards environment to the view.
     *
     * @return Api 
     */
    public function __construct ()
    {
        $this->connection = new model\Connection();
        $this->view = new view\Result($this->connection);

        // Provide hook for connecting through controller to justify query
        $result = $this->view->collect();

        // Leak the data
        echo $result;
        return $this;
    }
}

new Api();

