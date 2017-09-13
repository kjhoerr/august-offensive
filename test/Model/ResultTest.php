<?php

declare(strict_types=1);

namespace AugustOffensive\Model;

use PHPUnit\Framework\TestCase;

/**
 * @covers Result
 */
final class ResultTest extends \PHPUnit\Framework\TestCase
{
    public function testCanBeCreated()
    {
        $this->assertInstanceOf(
            Result::class,
            new Result("TEST_SUCCESS", array("it worked"))
        );
    }

    public function testHasAccessibleValues()
    {
        $resultType = "VISIBLE";
        $result = array("array", "of", "values");
        $resultObject = new Result($resultType, $result);

        $this->assertEquals(
            $resultType,
            $resultObject->getResultType()
        );
        $this->assertEquals(
            $result,
            $resultObject->getResult()
        );
    }
}
