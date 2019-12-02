<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>Create new record in database</description>
   <name>Create New Employee Record</name>
   <tag></tag>
   <elementGuidId>92f5a8c7-ca19-4456-9be3-6eedaac0b146</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\&quot;name\&quot;:\&quot;${employee_name}\&quot;,\&quot;salary\&quot;:\&quot;${salary_amount}\&quot;,\&quot;age\&quot;:\&quot;${employee_age}\&quot;}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>http://dummy.restapiexample.com/api/v1/create</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>'Bappy'</defaultValue>
      <description></description>
      <id>5a9fdf93-7df8-4536-ad19-4c636880e50b</id>
      <masked>false</masked>
      <name>employee_name</name>
   </variables>
   <variables>
      <defaultValue>99999999</defaultValue>
      <description></description>
      <id>5a7f54d8-e37b-4aa3-abe2-fa64304547aa</id>
      <masked>false</masked>
      <name>salary_amount</name>
   </variables>
   <variables>
      <defaultValue>26</defaultValue>
      <description></description>
      <id>500e043f-676b-4a8d-9e81-3e95ca4b43a2</id>
      <masked>false</masked>
      <name>employee_age</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()



assertThat(response.getResponseText()).contains('Bappy')</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
