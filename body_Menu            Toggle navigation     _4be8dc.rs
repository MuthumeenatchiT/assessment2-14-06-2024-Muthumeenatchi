<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>body_Menu            Toggle navigation     _4be8dc</name>
   <tag></tag>
   <elementGuidId>63a04b14-94e8-4a2e-8cfa-0a0d074ef070</elementGuidId>
   <selectorCollection>
      <entry>
         <key>CSS</key>
         <value>body</value>
      </entry>
      <entry>
         <key>XPATH</key>
         <value>//body</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <smartLocatorCollection>
      <entry>
         <key>SMART_LOCATOR</key>
         <value>internal:text=&quot;Menu Toggle navigation Admissions Contact COMMON ELIGIBILITY TEST, JUNE 2024 Onl&quot;i</value>
      </entry>
   </smartLocatorCollection>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>body</value>
      <webElementGuid>b37e8026-98cd-4c79-96e7-e4e4fa86e054</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>






  
    
        
            
        
       
        
        
        
          Menu
            Toggle navigation
            
            
            
          
       
       
        
            
                

                Admissions

                Contact
            
         
        
       
       
    
  


    



    $('.menu-list .active').each(function(index, el) {
        $(el).removeClass('active');
    });
    var active_menu = $('#id_active_menu').val();
    $('#'+active_menu).addClass('active');

    function fileDownload(file_path, url) {
          $('#file_path').val(file_path);
          document.file_download_list.action = url;
          document.file_download_list.target='_blank';
          document.file_download_list.submit();
    }




    document.onreadystatechange = function() {
        if (document.readyState !== &quot;complete&quot;) {
        document.querySelector(
        &quot;body&quot;).style.visibility = &quot;hidden&quot;;
        document.querySelector(
        &quot;#loader&quot;).style.visibility = &quot;visible&quot;;
        } else {
        document.querySelector(
        &quot;#loader&quot;).style.display = &quot;none&quot;;
        document.querySelector(
        &quot;body&quot;).style.visibility = &quot;visible&quot;;
        }
    };










	
	

	
	


	
	
		








		COMMON ELIGIBILITY TEST, JUNE 2024
		Online Registration Form



		
		
		
			
			
				New Registration
				Already Registered / LOGIN

			
			

			New Registration
				
			

						
							Name of the Candidate: *
							
								
							
							 
						  
						  
							Mobile Number : *
							
								
							
							 
						  
						
							Nationality : *
							
								
  Indian

  NRI


							
							 
						  

						
							Aadhaar No : *
							
								
							
							 
						  
						
							Email Id : *
							
								
								(Note: All mail communication will be send to this email id. So, give active email id.) 
							

							 
						  
						
							Set Password : *
							
								
								
							
							 
						  
						
							Confirm Password : *
							
								
								
							
							 
						  

						  
					
					
				

				Already Registered / LOGIN
			

            
                
                
                    
                        Login
                    
                    
                        
                            
                                
                                    
                                    
                                
                            
                            
                                
                                    
                                    
                                
                            

							 
								 
									 Show Password
								 
								 
									Forgot Password
								 
                            

                            
                                
									
                                
                            
                        
                    
                
                
            
        
				
			
			
		



	
	
	
	





Forgot Password

  
    
      Email Id:*
      
    
    
        Submit
    
  




	
		
			© 2023 Bharathiar University, Developed by Karkuvel Infotech
		
	

  









$(&quot;.numbers&quot;).on('input', function(e) {
    //allowed => only numbers
    //return inputElm.val().replace(/[^0-9]/gi, '');
    var start = this.selectionStart, end = this.selectionEnd;
    this.value = $(this).val().replace(/[^0-9]/gi, '');
    this.setSelectionRange(start, end);
});

function displaySearchType(){
    var sel_tab = $('#sel_tab').val();
    if(sel_tab == '2'){
    	$(&quot;.old&quot;).trigger(&quot;click&quot;);
    }
    else{
    	$(&quot;.new&quot;).trigger(&quot;click&quot;);
    }
}
function displayAadhar(){
    var value = $(&quot;#id_nationality&quot;).val();
    if(value == 'NRI'){
    	$(&quot;.aadhar-view&quot;).hide();
    }else{
    	$(&quot;.aadhar-view&quot;).show();
    }
}

$(&quot;#id_nationality&quot;).change(function () {
  displayAadhar();
});
$(document).ready(function(){
    $('#stab').easyResponsiveTabs({
        type: 'default', //Types: default, vertical, accordion
        width: 'auto', //auto or any width like 600px
        fit: true, // 100% fit in a container
        closed: 'accordion', // Start closed if in accordion view
        tabidentify: 'hor_1', // The tab groups identifier
        activate: function (event) { // Callback function if tab is switched
            var $tab = $(this);
            var $info = $('#nested-tabInfo');
            var $name = $('span', $info);
            $name.text($tab.text());
            $info.show();
        }
    });

    $('.example-multiple').multiselect({
        buttonWidth: '100%',
        nonSelectedText: &quot;Doesn't Matter &quot;,
        includeSelectAllOption: true,
        enableClickableOptGroups: true,
    });

    $('.example-multiple-50').multiselect({
        buttonWidth: '49.5%',
        nonSelectedText: &quot;Doesn't Matter &quot;,
        includeSelectAllOption: true,
        enableClickableOptGroups: true,
    });

    $('.example-single').multiselect({
        buttonWidth: '100%',
        nonSelectedText: &quot;Doesn't Matter &quot;,
        includeSelectAllOption: true,
        enableFiltering: true,
        enableCaseInsensitiveFiltering: true,
    });

    displaySearchType();
});

$(&quot;body&quot;).on('click', '.login-password', function() {
  $(this).toggleClass(&quot;fa-eye fa-eye-slash&quot;);
  var input = $(&quot;#id_login_password&quot;);
  if (input.attr(&quot;type&quot;) === &quot;password&quot;) {
    input.attr(&quot;type&quot;, &quot;text&quot;);
  } else {
    input.attr(&quot;type&quot;, &quot;password&quot;);
  }
});


$('#chk_password').click(function() {
	var input = $(&quot;#id_password&quot;);
 	if ($('#chk_password').is(':checked')) {
		input.attr(&quot;type&quot;, &quot;text&quot;);
	}else{
		input.attr(&quot;type&quot;, &quot;password&quot;);
	}
})

$(&quot;body&quot;).on('click', '.login-confirm-password', function() {
  $(this).toggleClass(&quot;fa-eye fa-eye-slash&quot;);
  var input = $(&quot;#id_login_confirm_password&quot;);
  if (input.attr(&quot;type&quot;) === &quot;password&quot;) {
    input.attr(&quot;type&quot;, &quot;text&quot;);
  } else {
    input.attr(&quot;type&quot;, &quot;password&quot;);
  }
});

//open password dialog
function forgotPassword(){
	Custombox.open({
		target: '#password-modal',
		effect: 'contentscale',
		complete:   function() {
		  $(&quot;#id_email&quot;).focus();
		},
		close:   function() {
		},
	});
}
To Topid(&quot;id_name&quot;)</value>
      <webElementGuid>1e4f8386-a03e-4020-be4a-1c622bac9a14</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[1]/body[1]</value>
      <webElementGuid>abf0a14a-8fb2-4459-90fd-b7e214a665ed</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//body</value>
      <webElementGuid>f9e2a0c6-45ae-4d47-88f8-d2f6e7949ea8</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//body[(text() = concat(&quot;






  
    
        
            
        
       
        
        
        
          Menu
            Toggle navigation
            
            
            
          
       
       
        
            
                

                Admissions

                Contact
            
         
        
       
       
    
  


    



    $(&quot; , &quot;'&quot; , &quot;.menu-list .active&quot; , &quot;'&quot; , &quot;).each(function(index, el) {
        $(el).removeClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);
    });
    var active_menu = $(&quot; , &quot;'&quot; , &quot;#id_active_menu&quot; , &quot;'&quot; , &quot;).val();
    $(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot;+active_menu).addClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);

    function fileDownload(file_path, url) {
          $(&quot; , &quot;'&quot; , &quot;#file_path&quot; , &quot;'&quot; , &quot;).val(file_path);
          document.file_download_list.action = url;
          document.file_download_list.target=&quot; , &quot;'&quot; , &quot;_blank&quot; , &quot;'&quot; , &quot;;
          document.file_download_list.submit();
    }




    document.onreadystatechange = function() {
        if (document.readyState !== &quot;complete&quot;) {
        document.querySelector(
        &quot;body&quot;).style.visibility = &quot;hidden&quot;;
        document.querySelector(
        &quot;#loader&quot;).style.visibility = &quot;visible&quot;;
        } else {
        document.querySelector(
        &quot;#loader&quot;).style.display = &quot;none&quot;;
        document.querySelector(
        &quot;body&quot;).style.visibility = &quot;visible&quot;;
        }
    };










	
	

	
	


	
	
		








		COMMON ELIGIBILITY TEST, JUNE 2024
		Online Registration Form



		
		
		
			
			
				New Registration
				Already Registered / LOGIN

			
			

			New Registration
				
			

						
							Name of the Candidate: *
							
								
							
							 
						  
						  
							Mobile Number : *
							
								
							
							 
						  
						
							Nationality : *
							
								
  Indian

  NRI


							
							 
						  

						
							Aadhaar No : *
							
								
							
							 
						  
						
							Email Id : *
							
								
								(Note: All mail communication will be send to this email id. So, give active email id.) 
							

							 
						  
						
							Set Password : *
							
								
								
							
							 
						  
						
							Confirm Password : *
							
								
								
							
							 
						  

						  
					
					
				

				Already Registered / LOGIN
			

            
                
                
                    
                        Login
                    
                    
                        
                            
                                
                                    
                                    
                                
                            
                            
                                
                                    
                                    
                                
                            

							 
								 
									 Show Password
								 
								 
									Forgot Password
								 
                            

                            
                                
									
                                
                            
                        
                    
                
                
            
        
				
			
			
		



	
	
	
	





Forgot Password

  
    
      Email Id:*
      
    
    
        Submit
    
  




	
		
			© 2023 Bharathiar University, Developed by Karkuvel Infotech
		
	

  









$(&quot;.numbers&quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function(e) {
    //allowed => only numbers
    //return inputElm.val().replace(/[^0-9]/gi, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
    var start = this.selectionStart, end = this.selectionEnd;
    this.value = $(this).val().replace(/[^0-9]/gi, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
    this.setSelectionRange(start, end);
});

function displaySearchType(){
    var sel_tab = $(&quot; , &quot;'&quot; , &quot;#sel_tab&quot; , &quot;'&quot; , &quot;).val();
    if(sel_tab == &quot; , &quot;'&quot; , &quot;2&quot; , &quot;'&quot; , &quot;){
    	$(&quot;.old&quot;).trigger(&quot;click&quot;);
    }
    else{
    	$(&quot;.new&quot;).trigger(&quot;click&quot;);
    }
}
function displayAadhar(){
    var value = $(&quot;#id_nationality&quot;).val();
    if(value == &quot; , &quot;'&quot; , &quot;NRI&quot; , &quot;'&quot; , &quot;){
    	$(&quot;.aadhar-view&quot;).hide();
    }else{
    	$(&quot;.aadhar-view&quot;).show();
    }
}

$(&quot;#id_nationality&quot;).change(function () {
  displayAadhar();
});
$(document).ready(function(){
    $(&quot; , &quot;'&quot; , &quot;#stab&quot; , &quot;'&quot; , &quot;).easyResponsiveTabs({
        type: &quot; , &quot;'&quot; , &quot;default&quot; , &quot;'&quot; , &quot;, //Types: default, vertical, accordion
        width: &quot; , &quot;'&quot; , &quot;auto&quot; , &quot;'&quot; , &quot;, //auto or any width like 600px
        fit: true, // 100% fit in a container
        closed: &quot; , &quot;'&quot; , &quot;accordion&quot; , &quot;'&quot; , &quot;, // Start closed if in accordion view
        tabidentify: &quot; , &quot;'&quot; , &quot;hor_1&quot; , &quot;'&quot; , &quot;, // The tab groups identifier
        activate: function (event) { // Callback function if tab is switched
            var $tab = $(this);
            var $info = $(&quot; , &quot;'&quot; , &quot;#nested-tabInfo&quot; , &quot;'&quot; , &quot;);
            var $name = $(&quot; , &quot;'&quot; , &quot;span&quot; , &quot;'&quot; , &quot;, $info);
            $name.text($tab.text());
            $info.show();
        }
    });

    $(&quot; , &quot;'&quot; , &quot;.example-multiple&quot; , &quot;'&quot; , &quot;).multiselect({
        buttonWidth: &quot; , &quot;'&quot; , &quot;100%&quot; , &quot;'&quot; , &quot;,
        nonSelectedText: &quot;Doesn&quot; , &quot;'&quot; , &quot;t Matter &quot;,
        includeSelectAllOption: true,
        enableClickableOptGroups: true,
    });

    $(&quot; , &quot;'&quot; , &quot;.example-multiple-50&quot; , &quot;'&quot; , &quot;).multiselect({
        buttonWidth: &quot; , &quot;'&quot; , &quot;49.5%&quot; , &quot;'&quot; , &quot;,
        nonSelectedText: &quot;Doesn&quot; , &quot;'&quot; , &quot;t Matter &quot;,
        includeSelectAllOption: true,
        enableClickableOptGroups: true,
    });

    $(&quot; , &quot;'&quot; , &quot;.example-single&quot; , &quot;'&quot; , &quot;).multiselect({
        buttonWidth: &quot; , &quot;'&quot; , &quot;100%&quot; , &quot;'&quot; , &quot;,
        nonSelectedText: &quot;Doesn&quot; , &quot;'&quot; , &quot;t Matter &quot;,
        includeSelectAllOption: true,
        enableFiltering: true,
        enableCaseInsensitiveFiltering: true,
    });

    displaySearchType();
});

$(&quot;body&quot;).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;.login-password&quot; , &quot;'&quot; , &quot;, function() {
  $(this).toggleClass(&quot;fa-eye fa-eye-slash&quot;);
  var input = $(&quot;#id_login_password&quot;);
  if (input.attr(&quot;type&quot;) === &quot;password&quot;) {
    input.attr(&quot;type&quot;, &quot;text&quot;);
  } else {
    input.attr(&quot;type&quot;, &quot;password&quot;);
  }
});


$(&quot; , &quot;'&quot; , &quot;#chk_password&quot; , &quot;'&quot; , &quot;).click(function() {
	var input = $(&quot;#id_password&quot;);
 	if ($(&quot; , &quot;'&quot; , &quot;#chk_password&quot; , &quot;'&quot; , &quot;).is(&quot; , &quot;'&quot; , &quot;:checked&quot; , &quot;'&quot; , &quot;)) {
		input.attr(&quot;type&quot;, &quot;text&quot;);
	}else{
		input.attr(&quot;type&quot;, &quot;password&quot;);
	}
})

$(&quot;body&quot;).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;.login-confirm-password&quot; , &quot;'&quot; , &quot;, function() {
  $(this).toggleClass(&quot;fa-eye fa-eye-slash&quot;);
  var input = $(&quot;#id_login_confirm_password&quot;);
  if (input.attr(&quot;type&quot;) === &quot;password&quot;) {
    input.attr(&quot;type&quot;, &quot;text&quot;);
  } else {
    input.attr(&quot;type&quot;, &quot;password&quot;);
  }
});

//open password dialog
function forgotPassword(){
	Custombox.open({
		target: &quot; , &quot;'&quot; , &quot;#password-modal&quot; , &quot;'&quot; , &quot;,
		effect: &quot; , &quot;'&quot; , &quot;contentscale&quot; , &quot;'&quot; , &quot;,
		complete:   function() {
		  $(&quot;#id_email&quot;).focus();
		},
		close:   function() {
		},
	});
}
To Topid(&quot;id_name&quot;)&quot;) or . = concat(&quot;






  
    
        
            
        
       
        
        
        
          Menu
            Toggle navigation
            
            
            
          
       
       
        
            
                

                Admissions

                Contact
            
         
        
       
       
    
  


    



    $(&quot; , &quot;'&quot; , &quot;.menu-list .active&quot; , &quot;'&quot; , &quot;).each(function(index, el) {
        $(el).removeClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);
    });
    var active_menu = $(&quot; , &quot;'&quot; , &quot;#id_active_menu&quot; , &quot;'&quot; , &quot;).val();
    $(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot;+active_menu).addClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);

    function fileDownload(file_path, url) {
          $(&quot; , &quot;'&quot; , &quot;#file_path&quot; , &quot;'&quot; , &quot;).val(file_path);
          document.file_download_list.action = url;
          document.file_download_list.target=&quot; , &quot;'&quot; , &quot;_blank&quot; , &quot;'&quot; , &quot;;
          document.file_download_list.submit();
    }




    document.onreadystatechange = function() {
        if (document.readyState !== &quot;complete&quot;) {
        document.querySelector(
        &quot;body&quot;).style.visibility = &quot;hidden&quot;;
        document.querySelector(
        &quot;#loader&quot;).style.visibility = &quot;visible&quot;;
        } else {
        document.querySelector(
        &quot;#loader&quot;).style.display = &quot;none&quot;;
        document.querySelector(
        &quot;body&quot;).style.visibility = &quot;visible&quot;;
        }
    };










	
	

	
	


	
	
		








		COMMON ELIGIBILITY TEST, JUNE 2024
		Online Registration Form



		
		
		
			
			
				New Registration
				Already Registered / LOGIN

			
			

			New Registration
				
			

						
							Name of the Candidate: *
							
								
							
							 
						  
						  
							Mobile Number : *
							
								
							
							 
						  
						
							Nationality : *
							
								
  Indian

  NRI


							
							 
						  

						
							Aadhaar No : *
							
								
							
							 
						  
						
							Email Id : *
							
								
								(Note: All mail communication will be send to this email id. So, give active email id.) 
							

							 
						  
						
							Set Password : *
							
								
								
							
							 
						  
						
							Confirm Password : *
							
								
								
							
							 
						  

						  
					
					
				

				Already Registered / LOGIN
			

            
                
                
                    
                        Login
                    
                    
                        
                            
                                
                                    
                                    
                                
                            
                            
                                
                                    
                                    
                                
                            

							 
								 
									 Show Password
								 
								 
									Forgot Password
								 
                            

                            
                                
									
                                
                            
                        
                    
                
                
            
        
				
			
			
		



	
	
	
	





Forgot Password

  
    
      Email Id:*
      
    
    
        Submit
    
  




	
		
			© 2023 Bharathiar University, Developed by Karkuvel Infotech
		
	

  









$(&quot;.numbers&quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function(e) {
    //allowed => only numbers
    //return inputElm.val().replace(/[^0-9]/gi, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
    var start = this.selectionStart, end = this.selectionEnd;
    this.value = $(this).val().replace(/[^0-9]/gi, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
    this.setSelectionRange(start, end);
});

function displaySearchType(){
    var sel_tab = $(&quot; , &quot;'&quot; , &quot;#sel_tab&quot; , &quot;'&quot; , &quot;).val();
    if(sel_tab == &quot; , &quot;'&quot; , &quot;2&quot; , &quot;'&quot; , &quot;){
    	$(&quot;.old&quot;).trigger(&quot;click&quot;);
    }
    else{
    	$(&quot;.new&quot;).trigger(&quot;click&quot;);
    }
}
function displayAadhar(){
    var value = $(&quot;#id_nationality&quot;).val();
    if(value == &quot; , &quot;'&quot; , &quot;NRI&quot; , &quot;'&quot; , &quot;){
    	$(&quot;.aadhar-view&quot;).hide();
    }else{
    	$(&quot;.aadhar-view&quot;).show();
    }
}

$(&quot;#id_nationality&quot;).change(function () {
  displayAadhar();
});
$(document).ready(function(){
    $(&quot; , &quot;'&quot; , &quot;#stab&quot; , &quot;'&quot; , &quot;).easyResponsiveTabs({
        type: &quot; , &quot;'&quot; , &quot;default&quot; , &quot;'&quot; , &quot;, //Types: default, vertical, accordion
        width: &quot; , &quot;'&quot; , &quot;auto&quot; , &quot;'&quot; , &quot;, //auto or any width like 600px
        fit: true, // 100% fit in a container
        closed: &quot; , &quot;'&quot; , &quot;accordion&quot; , &quot;'&quot; , &quot;, // Start closed if in accordion view
        tabidentify: &quot; , &quot;'&quot; , &quot;hor_1&quot; , &quot;'&quot; , &quot;, // The tab groups identifier
        activate: function (event) { // Callback function if tab is switched
            var $tab = $(this);
            var $info = $(&quot; , &quot;'&quot; , &quot;#nested-tabInfo&quot; , &quot;'&quot; , &quot;);
            var $name = $(&quot; , &quot;'&quot; , &quot;span&quot; , &quot;'&quot; , &quot;, $info);
            $name.text($tab.text());
            $info.show();
        }
    });

    $(&quot; , &quot;'&quot; , &quot;.example-multiple&quot; , &quot;'&quot; , &quot;).multiselect({
        buttonWidth: &quot; , &quot;'&quot; , &quot;100%&quot; , &quot;'&quot; , &quot;,
        nonSelectedText: &quot;Doesn&quot; , &quot;'&quot; , &quot;t Matter &quot;,
        includeSelectAllOption: true,
        enableClickableOptGroups: true,
    });

    $(&quot; , &quot;'&quot; , &quot;.example-multiple-50&quot; , &quot;'&quot; , &quot;).multiselect({
        buttonWidth: &quot; , &quot;'&quot; , &quot;49.5%&quot; , &quot;'&quot; , &quot;,
        nonSelectedText: &quot;Doesn&quot; , &quot;'&quot; , &quot;t Matter &quot;,
        includeSelectAllOption: true,
        enableClickableOptGroups: true,
    });

    $(&quot; , &quot;'&quot; , &quot;.example-single&quot; , &quot;'&quot; , &quot;).multiselect({
        buttonWidth: &quot; , &quot;'&quot; , &quot;100%&quot; , &quot;'&quot; , &quot;,
        nonSelectedText: &quot;Doesn&quot; , &quot;'&quot; , &quot;t Matter &quot;,
        includeSelectAllOption: true,
        enableFiltering: true,
        enableCaseInsensitiveFiltering: true,
    });

    displaySearchType();
});

$(&quot;body&quot;).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;.login-password&quot; , &quot;'&quot; , &quot;, function() {
  $(this).toggleClass(&quot;fa-eye fa-eye-slash&quot;);
  var input = $(&quot;#id_login_password&quot;);
  if (input.attr(&quot;type&quot;) === &quot;password&quot;) {
    input.attr(&quot;type&quot;, &quot;text&quot;);
  } else {
    input.attr(&quot;type&quot;, &quot;password&quot;);
  }
});


$(&quot; , &quot;'&quot; , &quot;#chk_password&quot; , &quot;'&quot; , &quot;).click(function() {
	var input = $(&quot;#id_password&quot;);
 	if ($(&quot; , &quot;'&quot; , &quot;#chk_password&quot; , &quot;'&quot; , &quot;).is(&quot; , &quot;'&quot; , &quot;:checked&quot; , &quot;'&quot; , &quot;)) {
		input.attr(&quot;type&quot;, &quot;text&quot;);
	}else{
		input.attr(&quot;type&quot;, &quot;password&quot;);
	}
})

$(&quot;body&quot;).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;.login-confirm-password&quot; , &quot;'&quot; , &quot;, function() {
  $(this).toggleClass(&quot;fa-eye fa-eye-slash&quot;);
  var input = $(&quot;#id_login_confirm_password&quot;);
  if (input.attr(&quot;type&quot;) === &quot;password&quot;) {
    input.attr(&quot;type&quot;, &quot;text&quot;);
  } else {
    input.attr(&quot;type&quot;, &quot;password&quot;);
  }
});

//open password dialog
function forgotPassword(){
	Custombox.open({
		target: &quot; , &quot;'&quot; , &quot;#password-modal&quot; , &quot;'&quot; , &quot;,
		effect: &quot; , &quot;'&quot; , &quot;contentscale&quot; , &quot;'&quot; , &quot;,
		complete:   function() {
		  $(&quot;#id_email&quot;).focus();
		},
		close:   function() {
		},
	});
}
To Topid(&quot;id_name&quot;)&quot;))]</value>
      <webElementGuid>1817d773-0dfe-4ba3-bd9f-9dcc900fc6ed</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
