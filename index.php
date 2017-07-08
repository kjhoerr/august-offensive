<?php

declare(strict_types=1);

namespace AugustOffensive;

// Borrowed and modified from PSR-4 Closure Example
spl_autoload_register(
    function ($class) {
        $prefix = 'AugustOffensive\\';
        $relative_class = substr($class, strlen($prefix));

        // find file in /private/ in respective namespace path
        $file = __DIR__ . '/private/' . str_replace('\\', '/', $relative_class) . '.php';

        // if the file exists, require it
        if (file_exists($file)) {
            require $file;
        }
    }
);

use AugustOffensive\View;
use AugustOffensive\Controller;

// configure content type before anything is output
header("Content-Type: application/" + View\Main::TYPE);

try {
    // initiate connection and build front-end
    $connection = Controller\Controller::initiateConnection();
    $view = new View\Main($connection);

    // get results of query from front-end
    $result = $view->generateResult();

    echo $result;
} catch (\Exception $err) {
    // catch all exceptions and let the controller generate the error
    $error = Controller\Controller::errorResult($err);

    // pass generated error result to output
    echo View\Main::generateOutput($error);
}
