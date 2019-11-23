$(document).ready(function(){
    M.AutoInit();
    $('#mainNav li .main-link').click(function(event){
        event.preventDefault();
        linkLocation = this.href;
        $("body").fadeOut(600, redirectPage);
    });
    function redirectPage() {
        window.location = linkLocation;
    }
    let preloaderChecker = true;    
    window.onload = function() {
        if (preloaderChecker){
            $('.preloader').fadeOut();
            preloaderChecker = false;
        }      
    };
    window.setTimeout(function (){
        if (preloaderChecker){
            $('.preloader').fadeOut();
            preloaderChecker = false;
        }
    }, 1000);
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

function createCustomModal(header, fillModalContent, isClose = true) {
    let string = '<div id="custom-modal" class="modal">'
        +'<div class="modal-content">'
            +'<h4>'+header+'</h4>'
            +'<p class="custom-modal-content"></p>'
        +'</div>'
        +'<div class="modal-footer">'
            +function (){
                let result = isClose == true ? '<a class="modal-close waves-effect waves-green btn-flat">Закрыть</a>' : '';
                return result;
            }()
        +'</div>'
    +'</div>';
    $(".custom-modals").html(string);
    let modalobject =  $('#custom-modal');
    modalobject.modal();
    modalobject.modal("open");
    fillModalContent();
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
    +'<a onclick="renderParameter()" class="waves-effect btn">Выбрать</a>';
    $('.custom-modal-content').html(string);
    $('select').formSelect();
}

function renderParameter(){
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
}

function logicParameterDisabled (numberUsedLogical, idParameter){
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
}

function buildParameters (){
    let parameters = $('#parametersTable>tbody>.parameters'),
    arrayObjectParams = [];

    $.each(parameters, function(index, value){
        
        let idParameter = value.getAttribute("data-id"),
        nameParam = $('.parameterName[data-id='+idParameter+']'),
        sortParam = $('.parameterSort[data-id='+idParameter+']'),
        prioritySortParam = $('.parameterPrioritySort[data-id='+idParameter+']'),
        sortOrderParam = $('.parameterSortOrder[data-id='+idParameter+']'),
        obligatoryParam = $('.parameterObligatory[data-id='+idParameter+']'),
        hiddenParam = $('.parameterHidden[data-id='+idParameter+']'),
        orderShowParam = $('.parameterOrderShow[data-id='+idParameter+']'),
        typeParam = $('.parameterType[data-id='+idParameter+']');
        objectParam = {
            name: nameParam.val(),
            type: typeParam.text(),
            sort: 0,
            prioritySort: 0,
            sortOrder: "asc",
            obligatory: 0,
            hidden: 0,
            orderShow: Number(orderShowParam.val())
        };
        if (sortParam.prop("checked") == true){
            objectParam.sort = 1;
            objectParam.prioritySort = prioritySortParam.val();
            if (sortOrderParam.prop("checked") == true){
                objectParam.sortOrder = "desc"
            }
        }
        if (obligatoryParam.prop("checked") == true){
            objectParam.obligatory = 1;
        }
        else if (hiddenParam.prop("checked") == true){
            objectParam.hidden = 1;
        }
        arrayObjectParams.push(objectParam);
    });
    let queue = {
        name: $("#name_queque").val(),
        parameters: JSON.stringify(arrayObjectParams) 
    };   
}