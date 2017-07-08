<?php

declare(strict_types=1);

namespace AugustOffensive\View;

use AugustOffensive\Controller;
use AugustOffensive\Model;

/**
 * Outputs the JSON result by communicating with the controller.
 */
class Main
{
    /** @var Model\Query $query the Query object that generated the request. */
    private $query;

    /** @var Model\Result $result the Result object to be sent to the client. */
    private $result;

    /**
     * Prepares the output and environment for the front end of the service.
     *
     * @param Model\Connection $connection View "Needs to know" model exists.
     *
     * @return Result
     */
    public function __construct (Model\Connection $connection)
    {
        header("Content-Type: application/json");

        $this->query = Controller\Controller::createQuery(
            explode('/', trim($_SERVER['PATH_INFO'] ?? '/api', '/')),
            $_SERVER['REQUEST_METHOD'],
            $this->generateContent($_SERVER['REQUEST_METHOD'])
        );
        
        return $this;
    }

    /**
     * Generates the content of the query based on the request type.
     *
     * @param string $request The request method on which to base the content.
     *
     * @return array
     */
    public function generateContent (string $request): array
    {
        $content;
        switch ($request) {
            case "GET": // GET should always be empty
            case "POST": // POST contains moves, account info, etc.
            default:
                $content = $_REQUEST;
        }
        return $content;
    }

    /**
     * Communicates with the controller to generate the JSON result.
     *
     * @return string
     */
    public function generateResult (): string
    {
        $this->result = Controller\Controller::createResult(
            "",
            array()
        );

        // generate result
        return json_encode(array(
            "Result-Type" => $this->result->getResultType(),
            "Content" => $this->result->getResult()
        ));
    }
}
