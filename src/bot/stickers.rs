#![allow(dead_code)]
use teloxide::{
    requests::{Requester, ResponseResult},
    types::{InputFile, Message},
    Bot,
};

pub async fn sticker_hug(bot: Bot, msg: Message) -> ResponseResult<()> {
    bot.send_sticker(
        msg.chat.id,
        InputFile::file_id(
            "CAACAgIAAxkBAAEluctk-K1ifcDystDLakriS_aU1wSDIgACAwIAAhZCawpxS_u7v8y6NjAE",
        ),
    )
    .await?;
    Ok(())
}

pub async fn sticker_kiss(bot: Bot, msg: Message) -> ResponseResult<()> {
    bot.send_sticker(
        msg.chat.id,
        InputFile::file_id(
            "CAACAgIAAxkBAAEln_Fk9KrjpXTWYeLi-xynbyw1Kl_wPAACBAIAAhZCawqh06yi3GPURTAE",
        ),
    )
    .await?;
    Ok(())
}

pub async fn sticker_hello(bot: Bot, msg: Message) -> ResponseResult<()> {
    bot.send_sticker(
        msg.chat.id,
        InputFile::file_id(
            "CAACAgIAAxkBAAEln_9k9Ks28yYgs86MLg2g7CiTyDEFDAAC9wEAAhZCawo59nBvtGN_xDAE",
        ),
    )
    .await?;
    Ok(())
}

pub async fn sticker_sad(bot: Bot, msg: Message) -> ResponseResult<()> {
    bot.send_sticker(
        msg.chat.id,
        InputFile::file_id(
            "CAACAgIAAxkBAAEloAFk9KtH425_FKXXyXqU4RiZl8mSQAACqgsAAnp8CErk7LLLjREREzAE",
        ),
    )
    .await?;
    Ok(())
}

pub async fn sticker_sleep(bot: Bot, msg: Message) -> ResponseResult<()> {
    bot.send_sticker(
        msg.chat.id,
        InputFile::file_id(
            "CAACAgIAAxkBAAEloAdk9Kt2r5lRU68g4D6bZR6EqMMgOgACtQwAAmTCSUrua9qpbBFejzAE",
        ),
    )
    .await?;
    Ok(())
}

pub async fn sticker_lame(bot: Bot, msg: Message) -> ResponseResult<()> {
    bot.send_sticker(
        msg.chat.id,
        InputFile::file_id(
            "CAACAgIAAxkBAAEloAlk9KujTfIfFvTNC5R2qn6kB6B9TQACAQIAAhZCawotBlJ7kvEiDDAE",
        ),
    )
    .await?;
    Ok(())
}

pub async fn sticker_angry(bot: Bot, msg: Message) -> ResponseResult<()> {
    bot.send_sticker(
        msg.chat.id,
        InputFile::file_id(
            "CAACAgIAAxkBAAEloAtk9Ku-uNdLsdwaYN_3FrjQwqnt1wACeAsAAuEp-EnCkZE2mxUwJTAE",
        ),
    )
    .await?;
    Ok(())
}

pub async fn sticker_devil(bot: Bot, msg: Message) -> ResponseResult<()> {
    bot.send_sticker(
        msg.chat.id,
        InputFile::file_id(
            "CAACAgIAAxkBAAEloA9k9Kv06WRkAgVmw1mA8lZXCH-klAAC_wEAAhZCawpJbtHv-FIkDzAE",
        ),
    )
    .await?;
    Ok(())
}

pub async fn sticker_flower(bot: Bot, msg: Message) -> ResponseResult<()> {
    bot.send_sticker(
        msg.chat.id,
        InputFile::file_id(
            "CAACAgIAAxkBAAEloBNk9Kwbx0LVF-7ZnqrGZjUv_pGgmwAC_QEAAhZCawqkr2GipuryyTAE",
        ),
    )
    .await?;
    Ok(())
}

pub async fn sticker_love(bot: Bot, msg: Message) -> ResponseResult<()> {
    bot.send_sticker(
        msg.chat.id,
        InputFile::file_id(
            "CAACAgIAAxkBAAEloBVk9Kw8itM-dpLXqgw_vA7I4DfCaQACBgIAAhZCawof81Hl9_3GOzAE",
        ),
    )
    .await?;
    Ok(())
}

pub async fn sticker_laugh(bot: Bot, msg: Message) -> ResponseResult<()> {
    bot.send_sticker(
        msg.chat.id,
        InputFile::file_id(
            "CAACAgIAAxkBAAEloD1k9K6M60zSOzJKrneMGQoq0rMiCQACGw4AAtAWwUkS-iZmSyVP3TAE",
        ),
    )
    .await?;
    Ok(())
}

pub async fn sticker_coming_soon(bot: Bot, msg: Message) -> ResponseResult<()> {
    bot.send_sticker(
        msg.chat.id,
        InputFile::file_id(
            "CAACAgIAAxkBAAEloT5k9Nlb2sCpahomuYzV75gNKbzE4QAClgoAAmXXSEqeC5Vjb_xP4DAE",
        ),
    )
    .await?;
    Ok(())
}

pub async fn sticker_party_animals(bot: Bot, msg: Message) -> ResponseResult<()> {
    let stickers = [
        // turtle
        "CAACAgIAAxkBAAEluSdk-JbvYGIPlWmm8TqaQWT2zH0r7wAC9gsAArFaAUrxSFX7RKSbuzAE",
        // cat
        "CAACAgIAAxkBAAEluS1k-JcnvhTpKxi0LrwwEO5N-fL9fQACCxMAAujW4hIMnebll-_T-TAE",
        // dog
        "CAACAgIAAxkBAAEluTFk-JdI3NM2x5BXWl5preZcOdLQBQACWQADrWW8FPS7RxeJ4S0JMAQ",
        // frog
        "CAACAgIAAxkBAAEluTNk-JdxDlrqhNEJ42xFklGpqxHC2QACxxkAAuCZ8EgQ05nTiGAPwTAE",
        // cow
        "CAACAgQAAxkBAAEluTVk-JeP1Wvl8pIF3-hfieeB5z1lLAACOhgAAqbxcR6cYA5lHoA_dDAE",
        // ghost
        "CAACAgIAAxkBAAEluTdk-Jeq3XW1NasUbNvUyUUkQOGmnwAC3QADMNSdEY1VJRWnGm6vMAQ",
        // chick
        "CAACAgIAAxkBAAEluTlk-JfHRw4ToiYB4VLz7Wf2pIFXfQACKBYAArAOoUuS9aoVZQ9R8TAE",
    ];

    for sticker in stickers {
        bot.send_sticker(msg.chat.id, InputFile::file_id(sticker))
            .await?;
    }
    Ok(())
}
