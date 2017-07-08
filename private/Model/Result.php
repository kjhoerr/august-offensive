<?php

declare(strict_types=1);

namespace AugustOffensive\Model;
/**
 * Result object for storing information to send back to the client.
 */
class Result 
{
    /** @var string $resultType the type of result to return to the client. */
    private $resultType;

    /** @var array $result */
    private $result;

    /**
     * Store result information.
     *
     * @param string $resultType The type of result to send back to the client.
     * @param array $result The result object to send back to the client.
     *
     * @return Result
     */
    public function __construct (string $resultType, array $result)
    {
        $this->resultType = $resultType;
        $this->result = $result;

        return $this;
    }

    /**
     * Returns the result type of the Result.
     *
     * @return string
     */
    public function getResultType (): string
    {
        return $this->resultType;
    }

    /**
     * Returns the result array of the Result.
     *
     * @return array
     */
    public function getResult (): array
    {
        return $this->result;
    }
}