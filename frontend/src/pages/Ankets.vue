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
                                <th>Наименование</th>
                                <th>Ключ</th>
                                <th>Ключ очереди</th>
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
            axios.get('/api/services/queue_constructor/v1/queue_constructor/get_profiles').then(function(value){
                $("#profileTable tbody").html("");
                value.data.forEach(function(value){
                    // Здесь разбираются данные анкеты
                    let key = Exonum.uint8ArrayToHexadecimal(new Uint8Array(value.key.data));
                    // Создаем строку в таблице
                    $("#profileTable tbody").append("<tr><td></td><td>"+key+"</td><td></td></tr>");
                });
                M.toast({html: 'Очереди возвращены'});
            });
        }
    }
}
</script>