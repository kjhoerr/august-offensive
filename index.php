<?php

declare(strict_types=1);

namespace AugustOffensive;

// y u no autoload
include 'private/Model/Connection.php';
include 'private/Model/Query.php';
include 'private/Model/Result.php';
include 'private/View/Main.php';
include 'private/Controller/Controller.php';

use AugustOffensive\View;
use AugustOffensive\Model;

// initiate connection and build front-end
$connection = new Model\Connection();
$view = new View\Main($connection);

// get results of query from front-end
$result = $view->generateResult();

echo $result;
