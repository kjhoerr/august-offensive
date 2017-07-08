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

    /** @var string TYPE The type of output the client should receive. */
    public const TYPE = "json";

    /**
     * Prepares the output and environment for the front end of the service.
     *
     * @param Model\Connection $connection View "Needs to know" model exists.
     *
     * @return Result
     */
    public function __construct (Model\Connection $connection)
    {
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
     * Communicates with the controller to generate the output.
     *
     * @return string
     */
    public function generateResult (): string
    {
        switch (strtolower($this->query->getPath()[1])) {
            case "callback":
                $this->result = Controller\Controller::createResult(
                    "CALLBACK",
                    array(
                        "path" => $this->query->getPath(),
                        "request" => $this->query->getRequest(),
                        "content" => $this->query->getContent()
                    )
                );
                break;
            default:
                $this->result = Controller\Controller::createResult("", array());
                break;
        }

        return self::generateOutput($this->result);
    }

    /**
     * Creates output of the result based on the defined constant TYPE.
     *
     * @param Model\Result $result The result to be sent to the client.
     *
     * @return string
     */
    public static function generateOutput (Model\Result $result): string
    {
        $type = self::TYPE;
        return Output::$type($result);
    }
}
