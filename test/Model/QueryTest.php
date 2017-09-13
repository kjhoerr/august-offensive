<?php

declare(strict_types=1);

namespace AugustOffensive\Model;

use PHPUnit\Framework\TestCase;

/**
 * @covers Query
 */
final class QueryTest extends \PHPUnit\Framework\TestCase
{
    public function testCanBeCreated()
    {
        $this->assertInstanceOf(
            Query::class,
            new Query(array("api", "path"), "GET", array())
        );
    }

    public function testHasAccessibleValues()
    {
        $path = array("api", "query", "path");
        $request = "PUT";
        $content = array("a" => "apple", "b" => "bearing");
        $query = new Query($path, $request, $content);

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
}
