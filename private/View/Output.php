<?php

declare(strict_types=1);

namespace AugustOffensive\View;

use AugustOffensive\Model;

/**
 * Output formats for Results that are returned to the client.
 */
class Output
{
    /**
     * Formats the result into JSON.
     *
     * @param Model\Result $result The result to return to the client.
     * @param bool $prepare Whether to set the content type of the response (default true).
     *
     * @return string
     */
    public static function json (Model\Result $result, bool $prepare = true): string
    {
        // breaks side effect rule?
        if ($prepare) {
            header("Content-Type: application/json");
        }

        return json_encode(array(
            "Result-Type" => $result->getResultType(),
            "Content" => $result->getResult()
        ));
    }
}
