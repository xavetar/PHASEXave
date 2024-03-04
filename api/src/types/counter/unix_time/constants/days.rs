/*
 * Copyright 2023 Stanislav Mikhailov (xavetar)
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
 * THE SOFTWARE.
 */

/*

    День старта Unix time неизменнен, т.к день не зависит от представления даты, количество дней не изменно и
    не зависимо от календаря их формул.
    Любая дата - это абстракция, как и календарь, но день эры это абсолютная величина.

    Согласно любому календарю день текущей эры после которого начинается отсчёт unix time - это 719162*.
    Представление же дня эры* в Юлианском: 18.12.1969, в Григорианском: 31.12.1969, в Солнечном: 01.01.1970.

    Стоит отметить, что Юлианское представление даты, содержит в себе 2 дня с BCE. 03.01.1 CE (Current era) по
    Юлианскому календарю в своём представлении соответствует представлению 01.01.1 в Григорианском и Солнечном.
    Если убрать из представления дни с BCE 1 год Юлианского календаря будет состоять из 363 дням, а Январь будет
    содержать 29 дней. Эти BCE дни (01.01.1 BCE и 02.01.1 BCE) не должны включаться в расчёт дней текущей эры (CE).
    Вопрос заключается в другом, допустимо ли использовать в представлении Юлианского календаря дни из BCE?

    Таким образом фактически Юлианский календарь в текущей эре ДОЛЖЕН стартовать с Понедельника - 01.01.1 CE,
    а получается с Понедельника - 03.01.1 CE (где 01.01.1 BCE и 02.01.1 BCE), что является мягко говоря
    противоречивым, ведь эти два дня представления BCE находятся в представлении Юлианской даты CE.

    Этих дней в идеале, не должно быть в представлении даты Юлианского календаря для CE, но в теории
    для сохранения обратной совместимости - это может быть допустимым, но не должно быть запутывающим.
    Ответа на вопрос почему было сделано так, нигде нет - его удалось понять только при попытке исправить
    существующий алгоритм, ведь год состоящий из 363 дней, ломает совместимость и противоречит "модели"
    года состоящего из 365/366 дней и ломает все существующие алгоримты (кроме этого, ахаха).

    В идеальной реализации этого крейта, 1 год для Юлианского календаря должен состоять из 363 дней, а Январь
    из 29 дней, ведь мы можем учитывать в представлении даты CE, дни находящиеся в BCE - это не вопрос удобства,
    а вопрос логики, но при этом последующие годы будут идти в той-же модели из 365/366 дней, а дата после которой
    начнёт считать unix time так и останется 18.12.1969 в Юлианском представлении даты.

    Практически сделать это просто, но это увеличит количество кода, чтобы сохранить полную совместимость с BCE
    и летоисчиление продолжилось, и сохранть при этом полную совместимость Юлианского представления даты BCE и CE
    и не делая первый год состоящим из 363 дней, а Январь из 29 дней и сохранить совместимость с другими алгоритмами
    данные изменения ПОКА вноситься не будут.

*/

pub const UNIX_TIME_START_AFTER_DAY: u128 = 719162_u128; // 365.2425 * 1969 = 719162.4825