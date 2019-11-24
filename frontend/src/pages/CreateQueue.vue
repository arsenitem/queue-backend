<template>
<div>
    <div class="row">
        <div class="col s12 xl10 offset-xl1">
            <div class="row">
                <h3>Создание очереди</h3>
                <div class="row">
                    <form class="col s12">
                        <div class="row">
                            <div class="input-field col s12 xl4">
                                <input placeholder="Не указано" id="name_queque" type="text" class="validate">
                                <label for="name_queque">Наименование очереди</label>
                            </div>
                            <div class="input-field col s12 xl4">
                                <input type="text" placeholder="Не указано" id="date_start" class="datepicker">
                                <label for="date_start">Дата открытия</label>
                            </div>
                            <div class="input-field col s12 xl4">
                                <input type="text" placeholder="Не указано" id="date_end" class="datepicker">
                                <label for="date_end">Дата закрытия</label>
                            </div>                            
                        </div>
                        <div class="row custom-input">
                            <table id="parametersTable" class="responsive-table centered">
                                <thead>
                                    <tr>
                                        <th>Имя параметра</th>
                                        <th>Учавствует ли в сортировке?</th>
                                        <th>Коэффицент рейтинга</th>
                                        <th>По убывание, возрастанию</th>
                                        <th>Обязательный</th>
                                        <th>Скрыто</th>
                                        <th>Порядок отображения</th>
                                    </tr>
                                </thead>                       
                                <tbody>                         
                                </tbody>
                            </table>
                        </div>      
                        <a class="waves-effect waves-light btn" v-on:click="createCustomModal" ><i class="material-icons left">add_circle_outline</i>Добавить параметр</a>
                        <a class="waves-effect waves-light btn" v-on:click="buildParameters"><i class="material-icons left">play_circle_filled</i>Создать очередь</a>
                    </form>
                </div>  
                <div class="row">
                    <h3>Кнопки для тестирования</h3>
                    <div class=" col s12">
                        <div class="input-field col s12 xl4">
                            <input placeholder="Пусто" id="key_queque" type="text" class="validate">
                            <label for="key_queque">Ключ очереди</label>
                        </div>
                        <a class="waves-effect waves-light btn" v-on:click="transactionSaveQueue" ><i class="material-icons left">add_circle_outline</i>Создать тестовую очередь</a>
                        <a class="waves-effect waves-light btn" v-on:click="transactionGetQueue" ><i class="material-icons left">add_circle_outline</i>Вернуть мне очередь по ключу</a>
                        <a class="waves-effect waves-light btn" v-on:click="transactionSaveTestParameterQueue" ><i class="material-icons left">add_circle_outline</i>Создать атрибут очереди</a>
                    </div>
                </div>             
            </div>
        </div>
    </div> 
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

    function CreateTransaction(publicKey) {
        return Exonum.newTransaction({
            author: publicKey,
            service_id: 10,
            message_id: 0,
            schema: proto.queue_constructor.CreateQueue
        })
    }
    function CreateTransactionAttr(publicKey) {
        return Exonum.newTransaction({
            author: publicKey,
            service_id: 10,
            message_id: 1,
            schema: proto.queue_constructor.AddAttributesToQueue
        })     
    }

    function CreateTransactionUniversale(publicKey, message_id, schema) {
        return Exonum.newTransaction({
            author: publicKey,
            service_id: 10,
            message_id: message_id,
            schema: schema
        })     
    }

    module.exports = {
        methods: {          
            createCustomModal() {
                let string = '<div id="custom-modal" class="modal">'
                    +'<div class="modal-content">'
                        +'<h4>Выбрать тип очереди</h4>'
                        +'<p class="custom-modal-content"></p>'
                    +'</div>'
                    +'<div class="modal-footer">'
                        +'<a class="modal-close waves-effect waves-green btn-flat">Закрыть</a>'
                    +'</div>'
                +'</div>';
                $(".custom-modals").html(string);
                let modalobject =  $('#custom-modal');
                modalobject.modal();
                modalobject.modal("open");
                fillModalContentParameters();
            },
            buildParameters() {
                
                if ($("#name_queque").val() != ""){
                    let lengthIntParam = 1;
                    let parameters = $('#parametersTable>tbody>.parameters'),
                    arrayObjectParams = [];
                    // Describe transaction
                    var keyPairQueue = Exonum.keyPair();
                    const transaction = new CreateTransaction(keyPairQueue.publicKey);

                    // Transaction data
                    const data = {
                        name: $("#name_queque").val()
                    };
                    $("#name_queque").val(keyPairQueue.publicKey);
                    // Send transaction into blockchain        
                    var t = transaction.send(TRANSACTION_URL, data, keyPairQueue.secretKey);
                    t.then(function(value) {
                            $.ajax({
                                url: 'http://127.0.0.1:8280'+TRANSACTION_URL,
                                data: {hash: value},
                                success: function(result){
                                    if (result.status.type == "success"){
                                        $.each(parameters, function(index, value){                   
                                            let idParameter = value.getAttribute("data-id"),
                                            nameParam = $('.parameterName[data-id='+idParameter+']'),
                                            sortParam = $('.parameterSort[data-id='+idParameter+']'),
                                            prioritySortParam = $('.parameterPrioritySort[data-id='+idParameter+']'),
                                            sortOrderParam = $('.parameterSortOrder[data-id='+idParameter+']'),
                                            obligatoryParam = $('.parameterObligatory[data-id='+idParameter+']'),
                                            hiddenParam = $('.parameterHidden[data-id='+idParameter+']'),
                                            orderShowParam = $('.parameterOrderShow[data-id='+idParameter+']'),
                                            typeParam = $('.parameterType[data-id='+idParameter+']'),
                                            objectParam = {
                                                //queueKey: keyPairQueue.publicKey,
                                                name: nameParam.val(),
                                                /*typeAttribute: typeParam.text(),
                                                order: orderShowParam.val(),
                                                sortable: 0,
                                                priorityInOrder: false,
                                                obligatory: 0,
                                                coefficient: Number(prioritySortParam.val())*/
                                                
                                            };
                                            /*if (sortParam.prop("checked") == true){
                                                objectParam.sort = 1;
                                                objectParam.prioritySort = prioritySortParam.val();
                                                if (sortOrderParam.prop("checked") == true){
                                                    objectParam.sortOrder = 1
                                                }
                                            }
                                            if (obligatoryParam.prop("checked") == true){
                                                objectParam.obligatory = 1;
                                            }
                                            else if (hiddenParam.prop("checked") == true){
                                                objectParam.hidden = 1;
                                            }*/
                                            debugger;
                                            var keyPairParam = Exonum.keyPair();
                                            let transactionParam = new CreateTransactionAttr(keyPairQueue.publicKey);

                                            // Transaction data
                                            let dataParam = objectParam;
                                            // Send transaction into blockchain        
                                            let paramPromise = transactionParam.send(TRANSACTION_URL, dataParam, keyPairQueue.secretKey),
                                            lengthIntParams = parameters.length;
                                            paramPromise.then(function(valueParam){
                                                if (lengthIntParam != lengthIntParams){
                                                    lengthIntParam++;
                                                }
                                                else {
                                                    M.toast({html: 'Очередь и параметры сохранены!'});
                                                }                                                                                                                                       
                                            });
                                        });
                                    }                               
                                }
                            });
                        }, function(reason) {
                        // отказ
                    });
                }
                
                
                /*let queue = {
                    name: $("#name_queque").val(),
                    parameters: JSON.stringify(arrayObjectParams) 
                }*/
            },

            /* Методы для тестирования */
            transactionSaveQueue() {
                // Describe transaction
                var keyPair = Exonum.keyPair();
                const transaction = new CreateTransaction(keyPair.publicKey);

                // Transaction data
                const data = {
                    name: "Очередь 1"
                };

                $("#key_queque").val(keyPair.publicKey);
                // Send transaction into blockchain        
                var t = transaction.send(TRANSACTION_URL, data, keyPair.secretKey);
                t.then(function(value) {
                        $.ajax({
                            url: 'http://127.0.0.1:8280'+TRANSACTION_URL,
                            data: {hash: value},
                            success: function(result){
                                if (result.status.type == "success"){
                                    M.toast({html: 'Очередь сохранена!'});
                                }                               
                            }
                        });
                    }, function(reason) {
                    // отказ
                });
            },
            transactionGetQueue (){
                axios.get('/api/services/queue_constructor/v1/queue_constructor/get_queue?pub_key='+$("#key_queque").val()).then(function(value){
                    M.toast({html: 'Очередь возвращена'});
                });
            },

            transactionSaveTestParameterQueue(){
                var keyPair = Exonum.keyPair();
                const transaction = new CreateTransactionUniversale(keyPair.publicKey, 1, proto.queue_constructor.AddAttributesToQueue);
                let key = { data: Exonum.hexadecimalToUint8Array($("#key_queque").val()) };
                let dataParam = {
                    queueKey: key,
                    name: "Тестовый параметр",
                    typeAttribute: "string",
                    order: 1,
                    sortable: 0,
                    priorityInOrder: false,
                    obligatory: 0,
                    coefficient: 1                   
                };

                var t = transaction.send(TRANSACTION_URL, dataParam, keyPair.secretKey);
            }
        }
    }

    function fillModalContentParameters () {
        let string = '<div class="input-field col s12">'
            +'<select onclick="renderParametr()" id="selectParametrs">'
                +'<option value="" disabled selected>Не выбрано</option>'
                +'<option value="1">Логическое (да/нет)</option>'
                +'<option value="2">Текстовое</option>'
                +'<option value="3">Числовое</option>'
                +'<option value="4">Дата</option>'
            +'</select>'
            +'<label>Выберите тип параметра</label>'
        +'</div>'
        +'<a onclick="renderParameter()" id="selectParameter" class="waves-effect btn">Выбрать</a>';
        $('.custom-modal-content').html(string);
        $('select').formSelect();
        $("#selectParameter").click(function(){        
            let valueParametr = $('#selectParametrs').val(),
            rows = {
                nameParameter: "",
                sort: {
                    bool: false,
                    disabled: false
                },
                prioritySort: {
                    disabled: true
                },
                sortOrder: {
                    bool: false,
                    disabled: true
                },
                orderShow: {
                    disabled: false
                },
                obligatory: {
                    bool: false,
                    disabled: false
                },
                hidden: {
                    bool: false,
                    disabled: false
                },
                typeParameter: ""
            };
            switch(Number(valueParametr)) {
                case 1:
                    rows.typeParameter = "bool";
                    break;
                case 2:
                    rows.typeParameter = "string";
                    rows.sort.disabled = true;
                    break;
                case 3:
                    rows.typeParameter = "number";
                    break;
                case 4:
                    rows.typeParameter = "date";
                    break;
                default:    
                    $('#custom-modal').modal("close");  
                    return;      
                    break;
            }
            let rowsId = function(){return $(".parameters").length}(),
            tableRowHtml = '<tr data-id="'+rowsId+'" class="parameters">'
                +'<td>'             
                    +'<div class="input-field">'
                        +'<input type="text" data-id="'+rowsId+'" placeholder="Не указано" class="parameterName">'
                    +'</div>'
                +'</td>'
                +'<td>'
                    +'<p>'
                        +'<label>'
                            +'<input class="parameterSort" onclick="logicParameterDisabled(1, '+rowsId+')" data-id="'+rowsId+'" type="checkbox" '+function(){return rows.sort.disabled == true ? 'disabled' : ''}()+' />'
                            +'<span></span>'
                        +'</label>'
                    +'</p>'
                +'</td>'
                +'<td>'
                    +'<div class="input-field">'
                        +'<input data-id="'+rowsId+'" type="number" '+function(){return rows.prioritySort.disabled == true ? 'disabled' : ''}()+' value="'
                        + function(){return $(".parameters[disabled='']").length}()             
                        +'" class="parameterPrioritySort">'
                    +'</div>'
                +'</td>'
                +'<td>'
                    +'<div class="switch">'
                        +'<label>'
                            +'Возрастание'
                            +'<input data-id="'+rowsId+'" class="parameterSortOrder" '+function(){return rows.sortOrder.disabled == true ? 'disabled' : ''}()+' type="checkbox">'
                            +'<span class="lever"></span>'
                            +'Убывание'
                        +'</label>'
                    +'</div>'
                +'</td>'
                +'<td>'
                    +'<p>'
                        +'<label>'
                            +'<input data-id="'+rowsId+'" onclick="logicParameterDisabled(2, '+rowsId+')" class="parameterObligatory" type="checkbox" '+function(){return rows.obligatory.disabled == true ? 'disabled' : ''}()+' />'
                            +'<span></span>'
                        +'</label>'
                    +'</p>'
                +'</td>'
                +'<td>'
                    +'<p>'
                        +'<label>'
                            +'<input data-id="'+rowsId+'" onclick="logicParameterDisabled(3, '+rowsId+')" class="parameterHidden" type="checkbox" '+function(){return rows.hidden.disabled == true ? 'disabled' : ''}()+' />'
                            +'<span></span>'
                        +'</label>'
                    +'</p>'        
                +'</td>'
                +'<td>'
                    +'<div class="input-field">'
                        +'<input data-id="'+rowsId+'" type="number" '+function(){return rows.orderShow.disabled == true ? 'disabled' : ''}()+' value="'
                        + function(){return $(".parameters").length}()             
                        +'" class="parameterOrderShow">'
                    +'</div>'
                +'</td>'
                +'<td data-id="'+rowsId+'" style="display: none;" class="parameterType">'+rows.typeParameter+'</td>'
            +'</tr>';
            $('#parametersTable>tbody').append(tableRowHtml);
            $('#custom-modal').modal("close");
            $(".parameterSort, .parameterObligatory, .parameterHidden").click(function(){ 
                let numberUsedLogical = 0,
                idParameter = $(this).attr("data-id");
                if ($(this).prop("class") == "parameterSort"){
                    numberUsedLogical = 1;
                }
                else if ($(this).prop("class") == "parameterObligatory"){
                    numberUsedLogical = 2;
                }
                else if ($(this).prop("class") == "parameterHidden"){
                    numberUsedLogical = 3;
                }
                switch(numberUsedLogical){
                case 1:
                    if ($('.parameterSort[data-id='+idParameter+']').prop("checked")){
                        $('.parameterPrioritySort[data-id='+idParameter+']').prop('disabled', false);
                        $('.parameterSortOrder[data-id='+idParameter+']').prop('disabled', false);
                    }
                    else {
                        $('.parameterPrioritySort[data-id='+idParameter+']').val(0)
                        $('.parameterPrioritySort[data-id='+idParameter+']').prop('disabled', true);
                        $('.parameterSortOrder[data-id='+idParameter+']').prop('disabled', true);
                    }
                    $('.parameterPrioritySort[data-id='+idParameter+']')
                    break;
                case 2:
                    if ($('.parameterObligatory[data-id='+idParameter+']').prop("checked")){
                        $('.parameterHidden[data-id='+idParameter+']').prop('disabled', true);
                        $('.parameterHidden[data-id='+idParameter+']').prop("checked", false);
                    }
                    else {
                        $('.parameterHidden[data-id='+idParameter+']').prop('disabled', false);
                    }
                    break;
                case 3:
                    if ($('.parameterHidden[data-id='+idParameter+']').prop("checked")){
                        $('.parameterObligatory[data-id='+idParameter+']').prop('disabled', true);
                        $('.parameterObligatory[data-id='+idParameter+']').prop("checked", false);
                    }
                    else {
                        $('.parameterObligatory[data-id='+idParameter+']').prop('disabled', false);
                    }
                    break;
                }
            });
        });
    }

$('#createCustomModalButton').click(function(){
    createCustomModal('Добавить параметр', fillModalContentParameters);
});

function verificate (str){
    if (str != '' && str != null && str != undefined){
        return true;
    }
    return false;
}

function auth () {
    let login = $("#loginUser").val(),
    password = $("#passUser").val();
    if (verificate(login) && verificate(password)){
        M.toast({html: 'Успешно'});
    }
    else {
        M.toast({html: 'Заполните поля'});
    }
}

function tableSearch() {
    var phrase = document.getElementById('searchQueue');
    var table = document.getElementById('queueTable');
    var regPhrase = new RegExp(phrase.value, 'i');
    var flag = false;
    for (var i = 1; i < table.rows.length; i++) {
        flag = false;
        for (var j = table.rows[i].cells.length - 1; j >= 0; j--) {
            flag = regPhrase.test(table.rows[i].cells[j].innerHTML);
            if (flag) break;
        }
        if (flag) {
            table.rows[i].style.display = "";
        } else {
            table.rows[i].style.display = "none";
        }
    }
}

</script>