// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Russian locale.

use std::collections::HashMap;

pub fn messages() -> HashMap<&'static str, &'static str> {
    let mut m = HashMap::new();

    // Labels
    m.insert("label-translation", "Перевод:");
    m.insert("label-why", "Причина:");
    m.insert("label-fix", "Решение:");
    m.insert("label-link", "Ссылки:");

    // Categories
    m.insert("category-ownership", "Владение");
    m.insert("category-types", "Типы");
    m.insert("category-lifetimes", "Времена жизни");
    m.insert("category-borrowing", "Заимствование");

    // E0382 - Use of moved value
    m.insert("e0382-title", "Использование перемещённого значения");
    m.insert("e0382-translation", "заимствование перемещённого значения");
    m.insert(
        "e0382-explanation",
        "\
В Rust у каждого значения ровно один владелец в каждый момент времени.
Это основа гарантий безопасности памяти без сборщика мусора.

Когда вы присваиваете значение другой переменной или передаёте в функцию,
владение ПЕРЕМЕЩАЕТСЯ в новое место. Исходная переменная становится
недействительной и не может использоваться.

Это происходит потому, что Rust должен точно знать, когда освобождать память.
С одним владельцем нет неоднозначности в том, кто отвечает за очистку."
    );
    m.insert(
        "e0382-fix-clone-desc",
        "Клонировать значение (глубокое копирование)"
    );
    m.insert(
        "e0382-fix-borrow-desc",
        "Заимствовать через ссылку (без копирования)"
    );
    m.insert(
        "e0382-fix-copy-desc",
        "Реализовать трейт Copy (для маленьких типов)"
    );

    // E0502 - Cannot borrow as mutable (already borrowed as immutable)
    m.insert(
        "e0502-title",
        "Нельзя заимствовать как изменяемое (уже заимствовано как неизменяемое)"
    );
    m.insert(
        "e0502-translation",
        "нельзя заимствовать как изменяемое, т.к. уже заимствовано как неизменяемое"
    );
    m.insert(
        "e0502-explanation",
        "\
Rust применяет строгое правило заимствования: можно иметь ЛИБО одну
изменяемую ссылку, ЛИБО любое количество неизменяемых, но никогда оба сразу.

Это предотвращает гонки данных на этапе компиляции. Если бы можно было
изменять данные, пока кто-то их читает, читатель мог бы увидеть
несогласованное состояние.

Неизменяемое заимствование всё ещё \"активно\", потому что используется
дальше в коде. Rust отслеживает времена жизни ссылок."
    );
    m.insert(
        "e0502-fix-scope-desc",
        "Завершить неизменяемое заимствование перед мутацией"
    );
    m.insert("e0502-fix-clone-desc", "Клонировать данные перед мутацией");

    // E0499 - Cannot borrow as mutable more than once
    m.insert(
        "e0499-title",
        "Нельзя заимствовать как изменяемое больше одного раза"
    );
    m.insert(
        "e0499-translation",
        "нельзя заимствовать как изменяемое больше одного раза одновременно"
    );
    m.insert(
        "e0499-explanation",
        "\
Rust разрешает только ОДНУ изменяемую ссылку на данные одновременно.
Это строже правила неизменяемого заимствования и предотвращает любую
мутацию через алиасы.

Почему? Две изменяемые ссылки на одни данные могут привести к:
- Гонкам данных в параллельном коде
- Инвалидации итераторов
- Висячим указателям после реаллокации

Это правило проверяется на этапе компиляции, давая вам fearless concurrency."
    );
    m.insert(
        "e0499-fix-scope-desc",
        "Использовать области видимости для ограничения времени жизни ссылки"
    );
    m.insert(
        "e0499-fix-refcell-desc",
        "Использовать RefCell для interior mutability (проверки в рантайме)"
    );

    // E0308 - Mismatched types
    m.insert("e0308-title", "Несовпадение типов");
    m.insert("e0308-translation", "несовпадение типов");
    m.insert(
        "e0308-explanation",
        "\
Rust — статически типизированный язык, который НЕ выполняет неявные
преобразования типов. Каждое значение имеет конкретный тип, и компилятор
обеспечивает согласованность типов.

Это ловит ошибки на этапе компиляции, которые были бы ошибками времени
выполнения в других языках. Система типов — ваш друг, а не препятствие."
    );
    m.insert(
        "e0308-fix-convert-desc",
        "Использовать parse() для преобразования строки в число"
    );
    m.insert(
        "e0308-fix-as-desc",
        "Использовать 'as' для приведения числовых типов"
    );

    // E0106 - Missing lifetime specifier
    m.insert("e0106-title", "Отсутствует спецификатор времени жизни");
    m.insert(
        "e0106-translation",
        "отсутствует спецификатор времени жизни"
    );
    m.insert(
        "e0106-explanation",
        "\
Ссылки в Rust имеют времена жизни — они описывают, как долго ссылка валидна.
Обычно компилятор выводит времена жизни сам, но иногда нужно указать явно.

Аннотации времён жизни не изменяют то, как долго живут значения. Они
описывают связи между ссылками, чтобы компилятор мог проверить безопасность."
    );
    m.insert(
        "e0106-fix-lifetime-desc",
        "Добавить явный параметр времени жизни"
    );
    m.insert(
        "e0106-fix-owned-desc",
        "Использовать владеющий тип вместо ссылки"
    );

    // E0597 - Value does not live long enough
    m.insert("e0597-title", "Значение живёт недостаточно долго");
    m.insert("e0597-translation", "значение живёт недостаточно долго");
    m.insert(
        "e0597-explanation",
        "\
Вы создаёте ссылку на что-то, что будет уничтожено до того, как ссылка
будет использована. Это создало бы висячий указатель.

Rust предотвращает это на этапе компиляции. Значение, на которое ссылаются,
должно жить как минимум столько же, сколько сама ссылка."
    );
    m.insert(
        "e0597-fix-move-desc",
        "Переместить значение во внешнюю область видимости"
    );
    m.insert(
        "e0597-fix-owned-desc",
        "Вернуть владеющее значение вместо ссылки"
    );

    m
}

pub fn translations() -> HashMap<&'static str, &'static str> {
    let mut m = HashMap::new();

    // Error headers
    m.insert("error", "ошибка");
    m.insert("warning", "предупреждение");
    m.insert("note", "примечание");
    m.insert("help", "подсказка");

    // E0382 - borrow of moved value
    m.insert(
        "borrow of moved value",
        "заимствование перемещённого значения"
    );
    m.insert("move occurs because", "перемещение происходит потому что");
    m.insert(
        "which does not implement the `Copy` trait",
        "который не реализует трейт `Copy`"
    );
    m.insert("value moved here", "значение перемещено здесь");
    m.insert(
        "value borrowed here after move",
        "значение заимствовано здесь после перемещения"
    );
    m.insert(
        "consider cloning the value if the performance cost is acceptable",
        "рассмотрите клонирование значения, если допустима потеря производительности"
    );

    // E0502 - cannot borrow as mutable
    m.insert("cannot borrow", "нельзя заимствовать");
    m.insert(
        "as mutable because it is also borrowed as immutable",
        "как изменяемое, т.к. уже заимствовано как неизменяемое"
    );
    m.insert(
        "immutable borrow occurs here",
        "неизменяемое заимствование здесь"
    );
    m.insert(
        "mutable borrow occurs here",
        "изменяемое заимствование здесь"
    );
    m.insert(
        "immutable borrow later used here",
        "неизменяемое заимствование используется здесь"
    );

    // E0499 - cannot borrow as mutable more than once
    m.insert(
        "as mutable more than once at a time",
        "как изменяемое больше одного раза одновременно"
    );
    m.insert(
        "first mutable borrow occurs here",
        "первое изменяемое заимствование здесь"
    );
    m.insert(
        "second mutable borrow occurs here",
        "второе изменяемое заимствование здесь"
    );
    m.insert(
        "first borrow later used here",
        "первое заимствование используется здесь"
    );

    // E0308 - mismatched types
    m.insert("mismatched types", "несовпадение типов");
    m.insert("expected", "ожидается");
    m.insert("found", "найдено");
    m.insert("expected type", "ожидаемый тип");
    m.insert("found type", "найденный тип");
    m.insert("this expression has type", "это выражение имеет тип");

    // E0106 - missing lifetime
    m.insert(
        "missing lifetime specifier",
        "отсутствует спецификатор времени жизни"
    );
    m.insert(
        "expected named lifetime parameter",
        "ожидается именованный параметр времени жизни"
    );

    // E0597 - does not live long enough
    m.insert("does not live long enough", "живёт недостаточно долго");
    m.insert(
        "borrowed value does not live long enough",
        "заимствованное значение живёт недостаточно долго"
    );
    m.insert(
        "dropped here while still borrowed",
        "удалено здесь, пока ещё заимствовано"
    );

    // Common phrases
    m.insert("has type", "имеет тип");
    m.insert("consider", "рассмотрите");
    m.insert(
        "this error originates in the macro",
        "эта ошибка возникла в макросе"
    );
    m.insert("run with", "запустите с");
    m.insert(
        "for more info about this issue",
        "для информации об этой ошибке"
    );
    m.insert("aborting due to", "прерывание из-за");
    m.insert("previous error", "предыдущей ошибки");
    m.insert("previous errors", "предыдущих ошибок");
    m.insert("could not compile", "не удалось скомпилировать");
    m.insert("due to", "из-за");

    m
}
