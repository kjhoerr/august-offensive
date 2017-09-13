<?php

declare(strict_types=1);

namespace AugustOffensive\View;

use PHPUnit\Framework\TestCase;
use AugustOffensive\Model;

/**
 * @covers Output
 */
final class OutputTest extends \PHPUnit\Framework\TestCase
{
    public function testJSONOutput()
    {
        $resultType = "JSON_ENCODED";
        $result = array("json", "1.6");
        $resultObject = new Model\Result($resultType, $result);

        // If JSON is properly formatted, we can decode and test the values
        $output = json_decode(Output::json($resultObject));

        $this->assertEquals(
            $resultType,
            $output["Result-Type"]
        );
        $this->assertEquals(
            $result,
            $output["Content"]
        );
    }
}