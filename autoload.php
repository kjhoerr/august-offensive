<?php

declare(strict_types=1);

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
