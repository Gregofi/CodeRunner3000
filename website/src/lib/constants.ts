export interface IPrograms {
	[key: string]: string;
}

export const defaultPrograms: IPrograms = {
	lua: [
		'local function fact(n)',
		'    if n == 0 then',
		'        return 1',
		'    else',
		'        return n * fact(n - 1)',
		'    end',
		'end',
		'',
		'print(fact(5))'
	].join('\n'),
	scheme: [
		'#lang racket',
		'',
		'(define (fact n)',
		'    (if (= n 0)',
		'        1',
		'        (* n (fact (- n 1)))))',
		'',
		'(display (fact 5))'
	].join('\n'),
	python: [
		'def fact(n):',
		'    if n == 0:',
		'        return 1',
		'    else:',
		'        return n * fact(n - 1)',
		'',
		'print(fact(5))'
	].join('\n'),
	shell: ['#!/bin/bash', '', "echo 'Hello, World!'"].join('\n'),
	c: [
		'#include <stdio.h>',
		'',
		'int fact(int n) {',
		'    if (n == 0) {',
		'        return 1;',
		'    } else {',
		'        return n * fact(n - 1);',
		'    }',
		'}',
		'',
		'int main() {',
		'    printf("%d\\n", fact(5));',
		'    return 0;',
		'}'
	].join('\n'),
	cpp: [
		'#include <iostream>',
		'',
		'int fact(int n) {',
		'    if (n == 0) {',
		'        return 1;',
		'    } else {',
		'        return n * fact(n - 1);',
		'    }',
		'}',
		'',
		'int main() {',
		'    std::cout << fact(5) << "\\n";',
		'    return 0;',
		'}'
	].join('\n'),
	haskell: ['fact 0 = 1', 'fact n = n * fact (n - 1)', '', 'main = putStrLn $ show $ fact 5'].join(
		'\n'
	)
};
