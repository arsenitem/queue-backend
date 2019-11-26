<template>
<div>
    <main>
    <div class="row">
        <div class="col s12 xl10 offset-xl1">
            <div class="row">
                <h3>Анкеты</h3>
                <a v-on:click="getAttr" class="waves-effect waves-light btn"><i class="material-icons left">sync</i>Обновить</a>
                <div class="col s12">
                    <table id="profileTable">
                        <thead>
                            <tr>
                                <th>Ключ</th>
                                <th>Пользователь</th>
                                <th>Рейтинг</th>
                            </tr>
                        </thead>               
                        <tbody>
                            
                        </tbody>
                    </table>
                </div>
            </div>
        </div>
    </div>    
</main>
</div>
</template>
<script>
M.AutoInit();
    import * as Exonum from 'exonum-client'
    import axios from 'axios'
    import * as proto from '../../proto/stubs.js'

    const TRANSACTION_URL = '/api/explorer/v1/transactions';
    const PER_PAGE = 10;
    const SERVICE_ID = 10;
    const TX_TRANSFER_ID = 0;
    const TX_ISSUE_ID = 1;
    const TX_WALLET_ID = 0;
    const TABLE_INDEX = 0;
module.exports = {
    methods: {
        getAttr(){
            let example = [
                {key:'135225be38ea632ba2d35c572652171c644ad9963e55fae7631bb1275aa2d050',user_key:'9822d50db9b31d7ac426e4c9a4039dcccb0bfc6ae73887e7cc29ed0c4da97375',rating: 100},
                {key:'f2e6063eab4051d202e9e2d075aeee2ca5082eacfc919f0b1e80bae1026d394f',user_key:'11154ce4f21d011ded5b88281ed943bfa213db8e2009bbc2e0cced869ca90e97',rating: 99},
                {key:'6ecfc498feab4694bf59e871a5deba9458b2bfb2f7192792edc226720d8e94f9',user_key:'12cfc498feab4694bf59e871a5deba9458b2bfb2f7192792edc226720d8e94f9',rating: 70},
                {key:'255225be38ea632ba2d123572652171c644ad9963e551237631bb1275aa2d050',user_key:'9822d5jfhdt31d7ac426e4c9a4039dcccb0bfc6ae73887e7cc29ed0c4da97375',rating: 65},
                {key:'f2e6063eab40f23d202e9e2d075aeee2ca5082eacfc919f0b1e80bae1026d394f',user_key:'11154fhcg51d011ded5b88281ed943bfa213db8e2009bbc2e0cced869ca90e97',rating: 40},
                {key:'fhr5c498feab4694bf59e871a5deba9458b2bfb2f7192792edc226720d8e94f9',user_key:'12cfc498feab4694ghdb5671a5deba9458b2bfb2f7192792edc226720d8e94f9',rating: 17},
            ]
            $("#profileTable tbody").empty();
            example.forEach(item => {
                  $("#profileTable tbody").append(`<tr><td>${item.key}</td><td>${item.user_key}</td><td>${item.rating}</td></tr>`);
            })
            // axios.get('/api/services/queue_constructor/v1/queue_constructor/get_profiles').then(function(value){
            //     $("#profileTable tbody").html("");
            //     value.data.forEach(function(value){
            //         // Здесь разбираются данные анкеты
            //         let key = Exonum.uint8ArrayToHexadecimal(new Uint8Array(value.key.data));
            //         // Создаем строку в таблице
            //         $("#profileTable tbody").append("<tr><td></td><td>"+key+"</td><td></td></tr>");
            //     });
            //     M.toast({html: 'Очереди возвращены'});
            // });
        }
    }
}
</script>