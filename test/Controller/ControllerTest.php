<?php

declare(strict_types=1);

namespace AugustOffensive\Controller;

use PHPUnit\Framework\TestCase;
use AugustOffensive\Model;

/**
 * Integration test: requires DB connection. Expect side effects (use test db if possible).
 *
 * @covers Controller
 */
final class ControllerTest extends \PHPUnit\Framework\TestCase
{
    public function testDBConnection()
    {
        try {
            $this->assertInstanceOf(
                Model\Connection::class,
                Controller::initiateConnection()
            );
        } catch (\PDOException $err) {
            $this->fail("Database not initialized correctly: " . $err->getMessage());
        }
    }

    public function testCreateQuery()
    {
        $path = array("api", "create", "query");
        $request = "DELETE";
        $content = array("c" => "cherry", "d" => "dike");
        $query = Controller::createQuery($path, $request, $content);

        $this->assertInstanceOf(
            Model\Query::class,
            $query
        );

        $this->assertEquals(
            $path,
            $query->getPath()
        );
        $this->assertEquals(
            $request,
            $query->getRequest()
        );
        $this->assertEquals(
            $content,
            $query->getContent()
        );
    }

    public function testCreateResult()
    {

        $resultType = "TYPE";
        $result = array("no", "values");
        $resultObject = Controller::createResult($resultType, $result);

        $this->assertInstanceOf(
            Model\Result::class,
            $resultObject
        );

        $this->assertEquals(
            $resultType,
            $resultObject->getResultType()
        );
        $this->assertEquals(
            $result,
            $resultObject->getResult()
        );
    }

    public function testErrorResult()
    {
        $message = "Oh no! Oops!";
        $errorResult = Controller::errorResult(new \Exception($message));

        $this->assertInstanceOf(
            Model\Result::class,
            $errorResult
        );

        $this->assertEquals(
            "ERROR",
            $errorResult->getResultType()
        );
        $this->assertEquals(
            array("error" => $message),
            $errorResult->getResult()
        );
    }
}
