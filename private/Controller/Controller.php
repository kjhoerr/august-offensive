<?php

declare(strict_types=1);

namespace AugustOffensive\Controller;

use AugustOffensive\Model;

/**
 * Static controller class for interfacing between the view and the model.
 */
class Controller
{
    /**
     * Initiates connection with the database.
     *
     * While errors are ideally handled by the controller, this instantiation
     * will likely throw a \PDOException which should be handled by the front-
     * end due to this being a fatal error and generally unrecoverable.
     *
     * @return Model\Connection
     */
    public static function initiateConnection (): Model\Connection
    {
        return new Model\Connection();
    }

    /**
     * Creates and returns a Query object.
     *
     * If the creation results in an error, a different query object is
     * returned with the error message.
     *
     * @param array $path The array that holds the original request structure.
     * @param string $request The request method made to the server.
     * @param array $content The content object sent by the request.
     *
     * @return Model\Query
     */
    public static function createQuery (
        array $path,
        string $request,
        array $content
    ): Model\Query {
        try {
            return new Model\Query($path, $request, $content);
        } catch (\Exception $err) {
            return new Model\Query(array(), "", array("ERROR" => $err->getMessage()));
        }
    }

    /**
     * Creates and returns a Result object.
     *
     * @param string $resultType The type of result to send back to the client.
     * @param array $result The result object to send back to the client.
     *
     * @return Model\Result
     */
    public static function createResult (
        string $resultType,
        array $result
    ): Model\Result {
        try {
            return new Model\Result($resultType, $result);
        } catch (\Exception $err) {
            return new Model\Result("ERROR", array($err->getMessage()));
        }
    }

    /**
     * Obtain the error result based on the exception that was thrown.
     *
     * @param \Exception $err the error that was thrown.
     *
     * @return Model\Result
     */
    public static function errorResult (\Exception $err): Model\Result
    {
        $errorType = "";
        // Juggle error: objective is to sort error type
        try {
            throw $err;
        } catch (\PDOException $e) {
            $errorType = "DATABASE_ERROR";
        } catch (\Exception $e) {
            $errorType = "ERROR";
        }

        return new Model\Result($errorType, array("error" => $err->getMessage()));
    }
}
