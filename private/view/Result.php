<?php

namespace AugustOffensive\view;

include '../controller/Controller.php';
include '../model/Connection.php';

use AugustOffensive\controller;
use AugustOffensive\model;

/**
 * Outputs the JSON result by communicating with the controller.
 */
class Result
{
    /**
     * Prepares the output and environment for the front end of the service.
     *
     * @param \Connection $connection "needs to know" model exists
     *
     * @return Result
     */
    public function __construct (Connection $connection)
    {
        header("Content-Type: application/json");
        //
    }

    /**
     * Communicates with the controller to generate the JSON result.
     *
     * @return array $result resulting sendback object generated from query.
     */
    public function collect ()
    {
        //
        return array();
    }
}
