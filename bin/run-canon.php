#!/usr/bin/env php
<?php
require __DIR__ . "/../canon/src/Netcarver/Textile/Parser.php";

$long_options = array(
    "toggle-line-wrap",
);

$rest_index = null;
$options = getopt("", $long_options, $rest_index);
array_splice($argv, 1, $rest_index - 1);

if (count($argv) != 2 || $argv[1] == "help" || $argv[1] == "--help" || $argv[1] == "-h") {
    echo "{$argv[0]} expects one argument:\n";
    $indentation = str_repeat(" ", strlen($argv[0]));
    echo "{$indentation} -h, --help, help: prints this message\n";
    echo "{$indentation} [filename]        reads and parses file\n";
    echo "{$indentation} -                 reads and parses stdin\n";
    echo "\n";
    echo "{$indentation} note that output will have one extra newline\n";
    exit;
}

if ($argv[1] == "-") {
    $input = stream_get_contents(STDIN);
} else {
    $input = file_get_contents($argv[1]);
}

$parser = new \Netcarver\Textile\Parser();

if (isset($options["toggle-line-wrap"]) && $options["toggle-line-wrap"]) {
    $parser->setLineWrap(false);
}

$output = $parser->parse($input) . "\n";

var_dump($options);
echo str_repeat("-", 60) . "\n";
var_dump($input);
echo str_repeat("-", 60) . "\n";
echo $output;
