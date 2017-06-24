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
    /** @var \Connection $connection the model database interface */
    private $connection;

    /** @var \Result $view the view interface that outputs result of the query */
    private $view;

    /**
     * Initiates database connection and forwards environment to the view.
     *
     * @return Api 
     */
    public function __construct ()
    {
        $connection = new model\Connection();
        $view = new view\Result($connection);

        // Provide hook for connecting through controller to justify query
        $result = $view->collect();

        // Leak the data
        echo $result;
    }
}

new Api();

