<?php

declare(strict_types=1);

namespace AugustOffensive\Model;

/**
 * Query object for storing relevant query information.
 */
class Query 
{
    /** @var array $path array of request structure. */
    private $path;

    /** @var string $request type of request made to the server. */
    private $request;

    /** @var array $content structure of information sent to the server. */
    private $content;

    /**
     * Store query information.
     *
     * @param array $path The array that holds the original request structure.
     * @param string $request The request method made to the server.
     * @param array $content The content object sent by the request.
     * 
     * @return Query
     */
    public function __construct (
        array $path, 
        string $request, 
        array $content
    ) {
        $this->path = $path;
        $this->request = $request;
        $this->content = $content;

        return $this;
    }

    /**
     * Returns the request path made by the client.
     * 
     * @return array 
     */
    public function getPath (): array
    {
        return $this->path;
    }

    /**
     * Returns the request type made by the client.
     * 
     * @return string
     */
    public function getRequest (): string
    {
        return $this->request;
    }

    /**
     * Returns the information that is built from outside the request path.
     *
     * @return array
     */
    public function getContent (): array
    {
        return $this->content;
    }
}
